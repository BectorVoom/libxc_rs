//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2332/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2332(t24585: f64, t7999: f64, t24574: f64, t27800: f64, t225: f64, t27805: f64, t11613: f64, t1191: f64, t1238: f64, t1241: f64, t1252: f64, t15802: f64, t1720: f64, t2155: f64, t24612: f64, t24757: f64, t24897: f64, t254: f64, t27784: f64, t27785: f64, t27786: f64, t27792: f64, t3631: f64, t4940: f64, t498: f64, t5055: f64, t53703: f64, t7348: f64, t8088: f64, t94779: f64, t94820: f64, t94867: f64, t94902: f64, t94942: f64, t94980: f64, t95026: f64, t95058: f64, t95087: f64, t95122: f64, t95150: f64, t95184: f64, t95224: f64, t95723: f64, t95752: f64, t95779: f64, t95817: f64) -> f64 {
    let t95824 = t7999 * t24585;
    let t95834 = 0.54831135561607547884e-2_f64 * t24574 * t27800;
    let t95836 = t27805 * t225;
    let t95844 = -2.0_f64 * t53703 * t2155 - 6.0_f64 * t27784 * t27785 * t15802 - 2.0_f64 * t11613 * t8088 - t94779 - t1238 * t1241 * (t94820 + t94867 + t94902 + t94942 + t94980 + t95026 + t95058 + t95087 + t95122 + t95150 + t95184 + t95224 + t95723 + t95752 + t95779 + t95817) + 0.48738787165873375895e-2_f64 * t95824 - 0.21932454224643019153e-1_f64 * t7999 * t24612 - 12.0_f64 * t1191 * t254 * t27786 - 6.0_f64 * t5055 * t24897 - t95834 - t27792 * t3631 - 2.0_f64 * t95836 * t1252 + 2.0_f64 * t4940 * t7348 * t498 + t1720 * t24757 * t498;
    t95844
}
