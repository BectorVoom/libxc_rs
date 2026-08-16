//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2914/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2914(t10632: f64, t5790: f64, t10655: f64, t17521: f64, t17423: f64, t2792: f64, t912: f64, t17422: f64, t2844: f64, t2842: f64, t17524: f64, t17528: f64, t42023: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60722 = t5790 * t10632;
    let t60741 = 0.32163958997385070134e2_f64 * t10655 * t17521;
    let t60744 = 4.0_f64 * t2792 * t17423 * t912;
    let t60745 = t17422 * t2844;
    let t60748 = 0.32163958997385070134e2_f64 * t2842 * t60745 * t912;
    let t60750 = 0.64327917994770140268e2_f64 * t10655 * t17524;
    let t60752 = 0.1034520258385468006e4_f64 * t42023 * t17528;
    (t60722, t60741, t60744, t60748, t60750, t60752)
}
