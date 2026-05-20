//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3323/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3323<F: Float>(t2439: F, t2440: F, t6072: F, t2444: F, t689: F, t11008: F, t14978: F, t1579: F, t1580: F, t18800: F, t2770: F, t2771: F, t2829: F, t51233: F, t51237: F, t51239: F, t51241: F, t51246: F, t51251: F, t51256: F, t51259: F, t51262: F, t51264: F, t51272: F, t6071: F, t62549: F, t62572: F, t62611: F, t62655: F, t62679: F, t62705: F, t62733: F, t62754: F, t62792: F, t62825: F, t62856: F, t62887: F, t62912: F, t62945: F, t62973: F, t63002: F, t63024: F, t63041: F, t865: F, t868: F) -> F {
    let t63050 = t2439 * t2440 * t6072;
    let t63053 = t689 * t2444 * t6072;
    let t63055 = F::cast_from(0.52039682876708176102e-1_f64) * t51233 - F::cast_from(0.10975748638225852664e-1_f64) * t62549 - F::cast_from(0.65854491829355115987e0_f64) * t18800 * t2829 - F::cast_from(0.39512695097613069591e1_f64) * t865 * t11008 * t6071 * t2771 - F::cast_from(0.520396828767081761e-2_f64) * t51237 + F::cast_from(0.52039682876708176102e-1_f64) * t51239 + F::cast_from(0.78059524315062264152e-1_f64) * t51241 - F::cast_from(0.13170898365871023197e1_f64) * t51272 * t1580 + F::cast_from(0.26341796731742046394e1_f64) * t865 * t2770 * t1579 * t14978 + F::cast_from(0.39274398764404314548e-3_f64) * t51246 - F::cast_from(0.43902994552903410656e-1_f64) * t51251 + F::cast_from(0.39029762157531132076e-1_f64) * t51256 - F::cast_from(0.21951497276451705328e-1_f64) * t62572 - F::cast_from(0.46263278077393568556e-2_f64) * t51259 + F::cast_from(0.46263278077393568556e-2_f64) * t51262 + F::cast_from(0.58537326070537880875e-1_f64) * t51264 - F::cast_from(0.65854491829355115987e0_f64) * t865 * t868 * (t62611 + t62655 + t62679 + t62705 + t62733 + t62754 + t62792 + t62825 + t62856 + t62887 + t62912 + t62945 + t62973 + t63002 + t63024 + t63041) + F::cast_from(0.65049603595885220126e-3_f64) * t63050 + F::cast_from(0.10975748638225852664e-1_f64) * t63053;
    t63055
}
