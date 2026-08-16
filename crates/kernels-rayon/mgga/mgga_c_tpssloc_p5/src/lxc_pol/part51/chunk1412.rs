//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1412/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1412(t115027: f64, t118454: f64, t118466: f64, t121279: f64, t121782: f64, t1484: f64, t16596: f64, t1877: f64, t23295: f64, t24191: f64, t24339: f64, t2522: f64, t25365: f64, t25374: f64, t26563: f64, t26744: f64, t31430: f64, t31434: f64, t31441: f64, t33476: f64, t4255: f64, t4303: f64, t6670: f64, t7114: f64, t7540: f64, t868: f64) -> f64 {
    let t121907 = 2.0_f64 * t115027 * t1877 * t25374 - 3.0_f64 * t118454 * t2522 * t7114 - 3.0_f64 * t118466 * t2522 * t7114 - 3.0_f64 * t121279 * t2522 * t7114 - t121782 * t1877 * t868 + 3.0_f64 * t1484 * t2522 * t31430 + 6.0_f64 * t16596 * t23295 * t24191 - 3.0_f64 * t16596 * t2522 * t31434 - t1877 * t24339 * t7540 - t1877 * t31434 * t4303 + 6.0_f64 * t23295 * t24191 * t25365 - 3.0_f64 * t24339 * t2522 * t33476 - 3.0_f64 * t2522 * t26744 * t31441 - 6.0_f64 * t26563 * t4255 * t6670;
    t121907
}
