//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1058/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1058<F: Float>(t27363: F, t33: F, t126017: F, t27799: F, t196: F, t197: F, t28230: F, t13426: F, t8461: F, t18227: F, t32110: F, t4248: F, t2322: F, t33581: F, t4254: F, t5517: F, t651: F, t8460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127227 = t33 * t27363;
    let t127284 = t27799 * t126017;
    let t127317 = t28230 * t196 * t197;
    let t127365 = t13426 * t8461;
    let t127368 = t18227 * t8461;
    let t127370 = t4248 * t32110;
    let t127372 = t2322 * t33581;
    let t127374 = t4254 * t33581;
    let t127377 = t651 * t5517 * t8460;
    (t127227, t127284, t127317, t127365, t127368, t127370, t127372, t127374, t127377)
}
