//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2005/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2005<F: Float>(t25310: F, t25331: F, t2435: F, t25339: F, t11064: F, t7086: F, t1113: F, t2411: F, t530: F, t7311: F, t2470: F, t26049: F) -> (F, F, F, F, F, F) {
    let t93384 = t25310 * t25331;
    let t93391 = t2435 * t25339;
    let t93404 = t7086 * t11064;
    let t94245 = t2411 * t1113;
    let t94345 = t530 * t7311;
    let t94377 = t26049 * t2470;
    (t93384, t93391, t93404, t94245, t94345, t94377)
}
