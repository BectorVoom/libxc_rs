//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1285/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1285(t10610: f64, t3465: f64, t42934: f64, t38303: f64, t39116: f64, t39117: f64, t39121: f64, t40659: f64, t40672: f64, t43939: f64, t43943: f64, t45058: f64, t45060: f64, t45066: f64, t45068: f64, t45070: f64, t45073: f64) -> (f64, f64) {
    let t45078 = 3.0_f64 / 2.0_f64 * t10610 * t3465 * t42934;
    let t45079 = t45058 - t45060 - t39116 + 0.13680077012009379e-5_f64 * t40659 - 0.30487649791575028312e-3_f64 * t43939 + 0.43368970657079495308e-4_f64 * t43943 - t45066 - t45068 + t45070 - t45073 - 0.14088275218353950416e-1_f64 * t40672 - t39117 - 0.1440846329149835838e-2_f64 * t38303 + t39121 - t45078;
    (t45078, t45079)
}
