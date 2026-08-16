//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2352/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2352(t55921: f64, t7245: f64, t12571: f64, t27331: f64, t2240: f64, t29473: f64, t33: f64, t2110: f64, t26055: f64, t26070: f64, t26073: f64, t26076: f64, t26090: f64, t27308: f64, t27311: f64, t27341: f64, t6492: f64, t7435: f64, t7975: f64, t7978: f64, t96535: f64) -> f64 {
    let t104953 = t55921 * t7245;
    let t104958 = t12571 * t27331;
    let t104968 = t2240 * t33 * t29473;
    let t104971 = 2.0_f64 / 3.0_f64 * t26070 * t7978 + 2.0_f64 / 3.0_f64 * t26073 * t7978 + 2.0_f64 / 3.0_f64 * t26076 * t7978 + 2.0_f64 / 3.0_f64 * t7435 * t27308 + 2.0_f64 / 3.0_f64 * t7435 * t27311 + 5.0_f64 / 6.0_f64 * t104953 * t6492 + t96535 * t2110 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t104958 * t6492 + 2.0_f64 / 3.0_f64 * t26055 * t7975 + 5.0_f64 / 3.0_f64 * t27341 * t26090 + 2.0_f64 / 3.0_f64 * t26055 * t7978 + 5.0_f64 / 6.0_f64 * t104968 * t6492;
    t104971
}
