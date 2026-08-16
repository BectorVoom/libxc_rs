//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1128/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1128(t1577: f64, t3308: f64, t8034: f64, t3295: f64, t7524: f64, t10760: f64, t25670: f64, t6093: f64, t25307: f64, t19865: f64, t25314: f64, t261: f64, t3304: f64, t7239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39645 = t1577 * t3308 * t8034;
    let t39647 = t3295 * t7524;
    let t39650 = t6093 * t10760 * t25670;
    let t39655 = t6093 * t10760 * t25307;
    let t39658 = t19865 * t10760 * t25314;
    let t39661 = t3304 * t261 * t7239;
    (t39645, t39647, t39650, t39655, t39658, t39661)
}
