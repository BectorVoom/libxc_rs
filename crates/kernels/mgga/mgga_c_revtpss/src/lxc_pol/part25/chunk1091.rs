//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1091/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1091<F: Float>(t2394: F, t890: F, t2832: F, t2430: F, t1100: F, t3329: F, t10259: F, t93: F, t2037: F, t4168: F, t1455: F, t7337: F, t2045: F, t4153: F, t10301: F, t607: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51775 = t890 * t2394;
    let t51792 = t890 * t2832;
    let t51806 = t2430 * t890;
    let t52188 = t1100 * t3329;
    let t60551 = t93 * t10259;
    let t92556 = t2037 * t4168;
    let t92559 = t1455 * t7337;
    let t92563 = t4153 * t2045;
    let t92565 = t10301 * t607;
    (t51775, t51792, t51806, t52188, t60551, t92556, t92559, t92563, t92565)
}
