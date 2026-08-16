//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1159/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1159(t10903: f64, t11770: f64, t2201: f64, t37937: f64, t37940: f64, t37947: f64, t37951: f64, t37954: f64, t37957: f64, t39874: f64, t39879: f64, t39882: f64, t39887: f64, t39891: f64) -> f64 {
    let t39894 = t2201 * t10903 * t11770;
    let t39895 = 0.46574606203128791246e-1_f64 * t39894;
    let t39896 = 0.47609969197673950972e-2_f64 * t37937 + 0.2600466522016280569e0_f64 * t39874 + 0.14282990759302185292e-1_f64 * t37940 + 0.31147743054556651236e-1_f64 * t37947 + 0.93443229163669953708e-1_f64 * t37951 + 0.21831846657716620896e-2_f64 * t39879 + 0.22511059664845582436e0_f64 * t39882 - t39887 + 0.71414953796510926458e-2_f64 * t37954 + 0.23804984598836975486e-2_f64 * t37957 + 0.21831846657716620896e-2_f64 * t39891 - t39895;
    t39896
}
