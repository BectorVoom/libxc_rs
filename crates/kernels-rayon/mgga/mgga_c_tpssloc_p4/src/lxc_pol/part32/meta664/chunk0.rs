//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2095/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2095(t24574: f64, t27779: f64, t8015: f64, t85660: f64, t27826: f64, t27403: f64, t27389: f64, t8074: f64, t85917: f64, t24826: f64, t27511: f64, t15394: f64, t2127: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94700 = 0.18277045187202515961e-2_f64 * t24574 * t27779;
    let t94701 = t85660 * t8015;
    let t94710 = 0.54831135561607547884e-2_f64 * t24574 * t27826;
    let t94759 = 0.54831135561607547884e-2_f64 * t24574 * t27403;
    let t94779 = 0.18277045187202515961e-2_f64 * t24574 * t27389;
    let t94784 = t85917 * t8074;
    let t94787 = 0.54831135561607547884e-2_f64 * t24826 * t27511;
    let t94796 = t2127 * t221 * t15394;
    (t94700, t94701, t94710, t94759, t94779, t94784, t94787, t94796)
}
