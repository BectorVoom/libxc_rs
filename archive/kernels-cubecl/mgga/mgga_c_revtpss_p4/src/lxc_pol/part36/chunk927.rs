//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 927/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk927<F: Float>(t21090: F, t3625: F, t1263: F, t6573: F, t1038: F, t6593: F, t1244: F, t1241: F, t5273: F, t5292: F, t1260: F, t6601: F) -> (F, F, F, F, F, F) {
    let t21091 = t3625 * t21090;
    let t21093 = t1263 * t6573;
    let t21100 = t6593 * t1038;
    let t21101 = t1244 * t21100;
    let t21102 = t1241 * t21101;
    let t21107 = t5273 * t5292;
    let t21143 = t6601 * t1260;
    (t21091, t21093, t21100, t21102, t21107, t21143)
}
