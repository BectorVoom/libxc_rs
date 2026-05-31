//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1107/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1107<F: Float>(t10304: F, t2380: F, t10308: F, t2417: F, t41693: F, t41696: F, t41703: F, t41716: F, t41723: F, t41731: F, t41737: F, t41741: F, t41746: F, t41748: F, t41755: F, t41759: F, t9600: F) -> (F, F) {
    let t43236 = t10304 * t2380;
    let t43241 = F::cast_from(0.21397160493827160493e0_f64) * t41746 + F::cast_from(0.19257444444444444444e0_f64) * t41748 - F::cast_from(0.42794320987654320987e0_f64) * t41755 - F::cast_from(0.14443083333333333333e0_f64) * t41759 + F::cast_from(0.34663399999999999999e1_f64) * t41693 - F::cast_from(0.51995099999999999998e1_f64) * t41696 + F::cast_from(0.11554466666666666666e1_f64) * t41703 - F::cast_from(0.9628722222222222222e0_f64) * t41716 + F::cast_from(0.34663399999999999999e1_f64) * t41723 - F::cast_from(0.38514888888888888888e0_f64) * t41731 + F::cast_from(0.38514888888888888888e0_f64) * t41737 - F::cast_from(0.11554466666666666666e1_f64) * t41741 + F::cast_from(0.1056393e1_f64) * t43236 * t2417 - F::cast_from(0.469508e0_f64) * t10308 * t9600;
    (t43236, t43241)
}
