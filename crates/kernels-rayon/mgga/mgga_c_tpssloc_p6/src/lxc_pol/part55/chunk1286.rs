//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1286/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1286(t117926: f64, t118097: f64, t1238: f64, t14980: f64, t1760: f64, t1761: f64, t2145: f64, t2155: f64, t24567: f64, t24615: f64, t254: f64, t27395: f64, t27396: f64, t27406: f64, t27786: f64, t27792: f64, t32479: f64, t32499: f64, t32511: f64, t32520: f64, t34314: f64, t34322: f64, t34331: f64, t3487: f64, t3598: f64, t7283: f64, t7300: f64, t7351: f64, t7356: f64, t8002: f64, t8888: f64, t94656: f64, t95836: f64, t95899: f64, t95902: f64) -> f64 {
    let t125712 = -2.0_f64 * t95902 * t2155 + 4.0_f64 * t7351 * t27396 - 0.87729816898572076614e-1_f64 * t27406 * t32511 + 0.3289868133696452873e-1_f64 * t7283 * t7300 * t24615 * t27395 + 0.14621636149762012769e-1_f64 * t27406 * t32520 - 6.0_f64 * t3487 * t34331 + 2.0_f64 * t14980 * t8888 + 0.43864908449286038307e-1_f64 * t27406 * t32499 + 4.0_f64 * t27792 * t7356 - t118097 * t1761 - 2.0_f64 * t95836 * t2155 - 0.54831135561607547883e-2_f64 * t7283 * t117926 * t8002 + 2.0_f64 * t1238 * t3598 * t32479 * t1760 - 12.0_f64 * t2145 * t254 * t27786 + 0.3289868133696452873e-1_f64 * t7283 * t24567 * t34322 + 4.0_f64 * t3487 * t34314 - 2.0_f64 * t95899 * t2155 - 2.0_f64 * t94656 * t2155;
    t125712
}
