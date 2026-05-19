//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 560/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk560<F: Float>(t428: F, t7883: F, t1711: F, t6: F, t64: F, t1300: F, t1617: F, t1620: F, t1683: F, t1687: F, t1698: F, t1701: F, t1702: F, t1712: F, t2035: F, t3065: F, t399: F, t403: F, t7833: F, t7838: F, t7840: F, t7845: F, t7848: F, t7852: F, t7854: F, t7860: F, t7861: F, t7867: F, t7868: F, t7877: F, t7879: F) -> (F, F) {
    let t7884 = t7883 * t428;
    let t7888 = t1711 * t6;
    let t7889 = t64 * t7888;
    let t7894 = -F::cast_from(0.20676097475611486194e-3_f64) * t1617 * t7833 * t1620 - F::cast_from(0.82704389902445944777e-3_f64) * t7838 * t7840 + F::cast_from(0.41352194951222972388e-3_f64) * t7845 * t7848 - F::cast_from(0.7112856777411015585e-1_f64) * t7852 * t7854 + F::cast_from(0.48082059875423759229e-5_f64) * t7860 * t7861 - F::cast_from(0.22524046461801549353e0_f64) * t403 * t1683 - F::cast_from(0.42160609613301514757e-3_f64) * t7867 * t2035 * t7868 + F::cast_from(0.35564283887055077925e-1_f64) * t1687 * t399 + F::cast_from(0.84321219226603029514e-3_f64) * t403 * t1698 + F::cast_from(0.139529405678626752e0_f64) * t7877 * t3065 * t7879 + F::cast_from(0.11262023230900774676e0_f64) * t1300 * t1701 * t7884 + F::cast_from(0.35564283887055077925e-1_f64) * t7889 * t1701 * t1702 * t1712;
    (t7889, t7894)
}
