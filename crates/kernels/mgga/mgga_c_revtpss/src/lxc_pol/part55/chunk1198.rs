//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1198/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1198<F: Float>(t127370: F, t2322: F, t33581: F, t4254: F, t5517: F, t651: F, t8460: F, t1868: F, t7311: F, t1459: F, t34012: F, t1916: F, t32375: F) -> (F, F, F, F, F, F, F) {
    let t127371 = F::new(2.0) * t127370;
    let t127372 = t2322 * t33581;
    let t127373 = F::new(2.0) * t127372;
    let t127374 = t4254 * t33581;
    let t127375 = F::new(2.0) * t127374;
    let t127377 = t651 * t5517 * t8460;
    let t127378 = F::new(2.0) * t127377;
    let t127381 = t1868 * t7311;
    let t127453 = F::new(6.0) * t1459 * t34012;
    let t127455 = F::new(6.0) * t1916 * t32375;
    (t127371, t127373, t127375, t127378, t127381, t127453, t127455)
}
