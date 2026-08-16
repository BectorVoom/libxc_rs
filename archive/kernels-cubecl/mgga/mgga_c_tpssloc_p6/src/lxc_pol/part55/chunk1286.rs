//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1286/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1286<F: Float>(t117926: F, t118097: F, t1238: F, t14980: F, t1760: F, t1761: F, t2145: F, t2155: F, t24567: F, t24615: F, t254: F, t27395: F, t27396: F, t27406: F, t27786: F, t27792: F, t32479: F, t32499: F, t32511: F, t32520: F, t34314: F, t34322: F, t34331: F, t3487: F, t3598: F, t7283: F, t7300: F, t7351: F, t7356: F, t8002: F, t8888: F, t94656: F, t95836: F, t95899: F, t95902: F) -> F {
    let t125712 = -F::cast_from(2.0_f64) * t95902 * t2155 + F::cast_from(4.0_f64) * t7351 * t27396 - F::cast_from(0.87729816898572076614e-1_f64) * t27406 * t32511 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t7300 * t24615 * t27395 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t32520 - F::cast_from(6.0_f64) * t3487 * t34331 + F::cast_from(2.0_f64) * t14980 * t8888 + F::cast_from(0.43864908449286038307e-1_f64) * t27406 * t32499 + F::cast_from(4.0_f64) * t27792 * t7356 - t118097 * t1761 - F::cast_from(2.0_f64) * t95836 * t2155 - F::cast_from(0.54831135561607547883e-2_f64) * t7283 * t117926 * t8002 + F::cast_from(2.0_f64) * t1238 * t3598 * t32479 * t1760 - F::cast_from(12.0_f64) * t2145 * t254 * t27786 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t24567 * t34322 + F::cast_from(4.0_f64) * t3487 * t34314 - F::cast_from(2.0_f64) * t95899 * t2155 - F::cast_from(2.0_f64) * t94656 * t2155;
    t125712
}
