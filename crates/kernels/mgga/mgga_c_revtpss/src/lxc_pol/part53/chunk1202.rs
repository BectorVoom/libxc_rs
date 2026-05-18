//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1202/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1202<F: Float>(t13426: F, t8461: F, t18227: F, t32110: F, t4248: F, t2322: F, t33581: F, t4254: F, t5517: F, t651: F, t8460: F, t1868: F, t7311: F) -> (F, F, F, F, F, F, F) {
    let t127365 = t13426 * t8461;
    let t127366 = F::new(2.0) * t127365;
    let t127368 = t18227 * t8461;
    let t127369 = F::new(2.0) * t127368;
    let t127370 = t4248 * t32110;
    let t127371 = F::new(2.0) * t127370;
    let t127372 = t2322 * t33581;
    let t127373 = F::new(2.0) * t127372;
    let t127374 = t4254 * t33581;
    let t127375 = F::new(2.0) * t127374;
    let t127377 = t651 * t5517 * t8460;
    let t127378 = F::new(2.0) * t127377;
    let t127381 = t1868 * t7311;
    (t127366, t127369, t127371, t127373, t127375, t127378, t127381)
}
