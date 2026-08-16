//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 897/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk897(t16917: f64, t4039: f64, t2722: f64, t4044: f64, t3608: f64, t10595: f64, t13794: f64, t13803: f64, t13912: f64, t1435: f64, t16676: f64, t16902: f64, t16912: f64, t277: f64, t4038: f64, t4054: f64, t5065: f64, t7332: f64, t8393: f64, t95: f64, t999: f64) -> (f64, f64, f64, f64, f64) {
    let t16918 = t4039 * t16917;
    let t16919 = t2722 * t16918;
    let t16921 = t4044 * t16917;
    let t16922 = t3608 * t16921;
    let t16925 = 14.0_f64 / 27.0_f64 * t999 * t16902 + t13912 * t1435 / 2.0_f64 + t4054 * t5065 / 2.0_f64 + t13794 / 2.0_f64 + 2.0_f64 / 9.0_f64 * t13803 - t10595 / 9.0_f64 + t16676 + 0.51689762869806860992e-2_f64 * t95 * t277 * t16912 * t8393 + t7332 - t4038 * t16919 + 2.0_f64 / 3.0_f64 * t4038 * t16922;
    (t16918, t16919, t16921, t16922, t16925)
}
