//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1129/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1129(t10760: f64, t24714: f64, t6535: f64, t3295: f64, t7520: f64, t3308: f64, t6362: f64, t8030: f64, t2834: f64, t3344: f64, t3290: f64, t7301: f64) -> (f64, f64, f64, f64, f64) {
    let t39540 = t6535 * t10760 * t24714;
    let t39542 = t3295 * t7520;
    let t39545 = t6362 * t3308 * t8030;
    let t39548 = t2834 * t3344;
    let t39549 = 0.47609969197673950972e-2_f64 * t39548;
    let t39550 = t3290 * t7301;
    (t39540, t39542, t39545, t39549, t39550)
}
