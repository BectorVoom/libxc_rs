//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1084/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1084<F: Float>(t19102: F, t12967: F, t14784: F, t14785: F, t19104: F, t19111: F, t19192: F, t19207: F, t19543: F, t19545: F, t19549: F, t19551: F, t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12948: F, t19100: F, t19106: F, t19116: F, t19121: F, t19125: F, t19129: F, t19485: F, t19488: F, t19491: F, t19494: F, t19497: F, t19528: F, t21771: F, t21793: F) -> (F,) {
    let t21804 = 0.22954444444444444444e0 * t19102;
    let t21815 = -0.68863333333333333333e0 * t19104 - 0.57386111111111111112e0 * t19111 - 0.13892666666666666667e0 * t12967 - t14784 - t14785 - 0.11577222222222222222e0 * t19543 + 0.90302333333333333334e0 * t19545 + 0.264729375e1 * t19207 - 0.157790625e0 * t19549 + 0.6311625e0 * t19551 + 0.3529725e1 * t19192;
    let t21817 = -t21771 + 0.46308888888888888889e-1 * t19485 - 0.34731666666666666667e-1 * t19488 + 0.20839e0 * t19491 - 0.62517e0 * t19494 - 0.46308888888888888889e-1 * t19497 + 0.20659e1 * t19116 - 0.13772666666666666667e1 * t19121 - 0.34431666666666666667e0 * t19125 - 0.309885e1 * t19129 + t21793 + 0.31558125e0 * t19528 - 0.22954444444444444444e0 * t19100 + 0.37874833333333333334e1 * t19106 + 0.17215833333333333333e0 * t12931 + 0.11477222222222222222e0 * t12933 - 0.23154444444444444444e0 * t12935 + 0.69463333333333333333e-1 * t12937 + 0.23154444444444444444e-1 * t12939 - 0.34431666666666666666e0 * t12948 + t21804 + t21815;
    (t21817,)
}
