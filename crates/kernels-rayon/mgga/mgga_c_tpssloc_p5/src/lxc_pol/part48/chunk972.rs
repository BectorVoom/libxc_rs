//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 972/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk972(t113069: f64, t113123: f64, t114970: f64, t114977: f64, t114988: f64, t114992: f64, t115000: f64, t115012: f64, t115027: f64, t13487: f64, t1877: f64, t1914: f64, t193: f64, t202: f64, t23285: f64, t23295: f64, t2379: f64, t24191: f64, t24339: f64, t24344: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t31430: f64, t31434: f64, t31441: f64, t31448: f64, t4314: f64, t6665: f64, t7114: f64, t776: f64, t84766: f64, t84791: f64, t84800: f64, t8566: f64, t868: f64, t870: f64) -> f64 {
    let t115099 = -6.0_f64 * t4314 * t7114 * t114977 + 4.0_f64 * t1877 * t24344 * t113123 + 4.0_f64 * t1877 * t84800 * t31448 - t1877 * t84791 * t1914 - 2.0_f64 * t1877 * t24339 * t6665 - t1877 * t7114 * t23285 + 2.0_f64 * t1877 * t24344 * t114988 + 6.0_f64 * t2522 * t31430 * t776 + 12.0_f64 * t24191 * t23295 * t13487 + 2.0_f64 * t1877 * t115027 * t2749 + t193 * t202 * t114970 * t870 + 6.0_f64 * t4314 * t8566 * t2379 - 2.0_f64 * t1877 * t114992 * t868 - 6.0_f64 * t1877 * t84766 * t115012 + 3.0_f64 * t2522 * t8566 * t2553 - 6.0_f64 * t2522 * t7114 * t113069 - 3.0_f64 * t2522 * t7114 * t115000 - t1877 * t31434 * t2745 - 6.0_f64 * t2522 * t24339 * t31441 - 6.0_f64 * t2522 * t31434 * t13487;
    t115099
}
