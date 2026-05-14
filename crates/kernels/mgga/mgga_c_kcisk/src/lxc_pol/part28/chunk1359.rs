//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1359/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1359<F: Float>(t24035: F, t5054: F, t9679: F, t23021: F, t22295: F, t6713: F, t1772: F, t2447: F, t7278: F, t2464: F, t32935: F, t7261: F, t7274: F, t2469: F, t7268: F, t121156: F, t121236: F, t121246: F, t32921: F, t34122: F, t34125: F, t34137: F, t34148: F, t34182: F, t34225: F, t35097: F, t9649: F, t9664: F, t9667: F) -> (F, F, F, F, F, F, F) {
    let t121269 = t5054 * t9679 * t24035;
    let t121272 = t5054 * t9679 * t23021;
    let t121275 = t6713 * t9679 * t22295;
    let t121284 = t7278 * t2447 * t1772;
    let t121299 = t7261 * t32935 * t7274 * t2464;
    let t121304 = t7261 * t32935 * t2469 * t7268;
    let t121309 = 0.27636574074074074073e-2 * t121269 - 0.16581944444444444444e-1 * t121272 + 0.13265555555555555555e-1 * t121275 + 0.20833333333333333334e-1 * t9664 * t121236 + 0.62500000000000000002e-1 * t9664 * t121246 - 0.33950617283950617287e-1 * t121156 * t9667 - 0.69444444444444444447e-2 * t121284 * t9667 + 0.55555555555555555558e-1 * t34125 * t34148 + 0.11111111111111111112e0 * t34125 * t34137 + 0.55555555555555555558e-1 * t34125 * t34182 + 0.21444444444444444445e-1 * t34225 * t34182 - 0.8041666666666666667e-2 * t32921 * t35097 - 0.8041666666666666667e-2 * t9649 * t121299 - 0.8041666666666666667e-2 * t9649 * t121304 - 0.41666666666666666668e-1 * t34122 * t34137;
    (t121269, t121272, t121275, t121284, t121299, t121304, t121309)
}
