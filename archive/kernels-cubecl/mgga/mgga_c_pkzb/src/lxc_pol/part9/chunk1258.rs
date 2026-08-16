//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1258/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1258<F: Float>(t1134: F, t1143: F, t12315: F, t1306: F, t135: F, t158: F, t18258: F, t2112: F, t2118: F, t2120: F, t21236: F, t21239: F, t21251: F, t21255: F, t21257: F, t21324: F, t21329: F, t21331: F, t21333: F, t21346: F, t2145: F, t2149: F, t21964: F, t22124: F, t273: F, t2957: F, t2965: F, t307: F, t311: F, t6001: F, t6006: F, t6055: F, t7825: F, t7828: F, t786: F, t7884: F, t7892: F, t799: F, t805: F) -> F {
    let t22129 = t21236 + t21239 - t21324 - F::cast_from(3.0_f64) * t1306 * t7892 * t2149 - t21329 - t21331 - t21333 - t21251 + t21255 + t21257 + t135 * t273 * (F::cast_from(0.15805078039045227836e2_f64) * t307 * t18258 * t1143 * t6001 + F::cast_from(0.39512695097613069591e1_f64) * t1134 * t6006 - F::cast_from(0.65854491829355115987e0_f64) * t1134 * t6055 + F::cast_from(0.39512695097613069591e1_f64) * t307 * t2118 * t7884 * t799 - F::cast_from(0.11853808529283920877e2_f64) * t21346 * t12315 * t2145 + F::cast_from(0.39512695097613069591e1_f64) * t2957 * t2120 + F::cast_from(0.39512695097613069591e1_f64) * t786 * t7828 + F::cast_from(0.39512695097613069591e1_f64) * t2112 * t2965 + F::cast_from(0.65854491829355115987e0_f64) * t21964 * t158 * t311 + F::cast_from(0.79025390195226139182e1_f64) * t786 * t7825 + t22124) * t805;
    t22129
}
