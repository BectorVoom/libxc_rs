//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 899/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk899(t5267: f64, t7778: f64, t903: f64, t26144: f64, t5181: f64, t645: f64, t27326: f64, t7577: f64, t5898: f64, t2060: f64, t27136: f64, t30080: f64, t8410: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39535 = t903 * t7778 * t5267;
    let t39536 = 0.23948483403727617128e0_f64 * t39535;
    let t39538 = t26144 * t645 * t5181;
    let t39541 = t903 * t7577 * t27326;
    let t39544 = t903 * t7778 * t5898;
    let t39545 = 0.23948483403727617128e0_f64 * t39544;
    let t39547 = t903 * t2060 * t27136;
    let t39549 = t30080 * t8410;
    (t39536, t39538, t39541, t39545, t39547, t39549)
}
