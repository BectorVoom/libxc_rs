//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 710/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk710<F: Float>(t1878: F, t454: F, t142: F, t1809: F, t1832: F, t5504: F, t5519: F, t3268: F, t3276: F, t1697: F, t2610: F, t102: F, t2615: F, t411: F, t127: F, t3217: F, t3228: F, t3260: F, t3280: F, t3282: F, t3284: F, t3288: F, t3290: F, t5502: F, t5507: F, t5511: F, t5513: F, t5517: F, t5523: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7082 = t454 * t1878;
    let t7083 = t7082 * t142;
    let t7085 = t1809 * t1832;
    let t7093 = 1.2991222222222223 * t5504;
    let t7096 = 0.6495611111111111 * t5519;
    let t7100 = 0.3247805555555556 * t3268;
    let t7101 = 0.6495611111111111 * t3276;
    let t7102 = t1697 * t2610;
    let t7108 = 17.53815 * t102 * t2615 * t411;
    let t7109 = -1.95872 * t5502 - t7093 - 4.0 / 9.0 * t5507 + t5511 - 0.97936 * t5513 + t5517 + t7096 + t5523 - 0.97936 * t3217 - 2.0 / 9.0 * t3228 - 0.48968 * t3260 + t7100 - t7101 + t3280 - t3282 - t3284 - t3288 - t3290 + 5.87616 * t127 * t7102 * t411 - t7108;
    (t7082, t7083, t7085, t7093, t7096, t7100, t7101, t7102, t7108, t7109)
}
