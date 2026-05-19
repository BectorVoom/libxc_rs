//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1215/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1215<F: Float>(t18826: F, t21373: F, t4068: F, t4977: F, t80087: F, t80089: F, t80091: F, t80096: F, t88728: F, t88732: F, t88737: F, t88758: F, t88766: F, t89947: F, t89950: F, t89981: F) -> F {
    let t91307 = F::cast_from(0.86658499999999999998e0_f64) * t88758 + F::cast_from(0.19257444444444444444e1_f64) * t88766 - F::cast_from(0.42794320987654320987e0_f64) * t88728 - F::cast_from(0.14443083333333333333e0_f64) * t88732 - F::cast_from(0.34663399999999999999e1_f64) * t88737 - F::new(0.469508e0) * t4068 * t21373 + F::new(0.1056393e1) * t18826 * t4977 - F::cast_from(0.77029777777777777776e0_f64) * t80087 + F::cast_from(0.11554466666666666666e1_f64) * t80089 + F::cast_from(0.21397160493827160493e0_f64) * t80091 + F::cast_from(0.19257444444444444444e0_f64) * t80096 - F::new(0.44016375e0) * t89947 - F::new(0.352131e0) * t89950 + F::new(0.234754e0) * t89981;
    t91307
}
