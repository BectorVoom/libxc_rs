//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 887/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk887(t3080: f64, t5267: f64, t26291: f64, t5888: f64, t40724: f64, t15075: f64, t25441: f64, t13819: f64, t8358: f64, t8362: f64, t13823: f64, t291: f64, t38855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75848 = t3080 * t5267;
    let t75850 = 0.17961362552795712846e0_f64 * t26291 * t75848;
    let t75851 = t3080 * t5888;
    let t75853 = 0.17961362552795712846e0_f64 * t40724 * t75851;
    let t75859 = t25441 * t15075;
    let t75864 = t13819 * t8358;
    let t75866 = t13819 * t8362;
    let t75869 = t13823 * t38855 * t291;
    (t75848, t75850, t75851, t75853, t75859, t75864, t75866, t75869)
}
