//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 919/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk919<F: Float>(t13717: F, t13742: F, t13772: F, t13775: F, t13777: F, t13881: F, t13886: F, t13888: F, t13892: F, t13912: F, t13915: F, t13918: F, t13921: F, t13924: F, t13927: F, t13931: F, t13934: F, t15398: F, t15420: F, t9681: F, t9683: F, t9691: F) -> (F,) {
    let t15422 = 0.264729375e1 * t13772 - 0.157790625e0 * t13881 - 0.3529725e1 * t13775 - 0.17648625e1 * t13777 + 0.6311625e0 * t13886 + 0.31558125e0 * t13888 - 0.20839e0 * t13892 + 0.17215833333333333333e0 * t9681 + 0.11477222222222222222e0 * t9683 - 0.45908888888888888888e0 * t9691 + t15398 + 0.46308888888888888889e-1 * t13912 - 0.34731666666666666667e-1 * t13915 - 0.46308888888888888889e-1 * t13918 - 0.13892666666666666667e0 * t13921 + 0.20839e0 * t13924 + 0.83356e0 * t13927 + 0.37874833333333333334e1 * t13717 + 0.20839e0 * t13931 - 0.62517e0 * t13934 - 0.103295e1 * t13742 + t15420;
    (t15422,)
}
