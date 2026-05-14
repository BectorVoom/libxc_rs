//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1240/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1240<F: Float>(t1727: F, t8891: F, t2639: F, t51: F, t3448: F, t5384: F, t16399: F, t8996: F, t6966: F, t8968: F, t164: F, t17034: F, t1733: F, t179: F, t20037: F, t20057: F, t20085: F, t20118: F, t20212: F, t2600: F, t590: F, t592: F, t6939: F, t8909: F) -> (F, F, F) {
    let t24251 = t1727 * t8891;
    let t24253 = t2639 * t2639;
    let t24254 = t51 * t24253;
    let t24259 = t5384 * t3448;
    let t24269 = t16399 * t8996;
    let t24272 = t6966 * t8968;
    let t24274 = 0.80031500487063509016e-1 * t20037 + 0.20007875121765877254e-2 * t20057 - 0.16006300097412701803e-1 * t20085 + 0.20007875121765877254e-2 * t24251 - 0.42874018118069736972e-3 * t590 * t592 * t24254 * t164 - 0.56688979511669985553e-2 * t24259 + 0.17149607247227894789e-2 * t1733 * t179 * t8909 * t6939 + 0.51448821741683684367e-1 * t17034 * t179 * t2600 * t20212 + 0.32012600194825403606e-1 * t24269 + 0.16006300097412701803e-1 * t20118 + 0.40015750243531754508e-2 * t24272;
    (t24253, t24254, t24274)
}
