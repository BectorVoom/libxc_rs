//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1107/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1107(t10304: f64, t2380: f64, t10308: f64, t2417: f64, t41693: f64, t41696: f64, t41703: f64, t41716: f64, t41723: f64, t41731: f64, t41737: f64, t41741: f64, t41746: f64, t41748: f64, t41755: f64, t41759: f64, t9600: f64) -> (f64, f64) {
    let t43236 = t10304 * t2380;
    let t43241 = 0.21397160493827160493e0_f64 * t41746 + 0.19257444444444444444e0_f64 * t41748 - 0.42794320987654320987e0_f64 * t41755 - 0.14443083333333333333e0_f64 * t41759 + 0.34663399999999999999e1_f64 * t41693 - 0.51995099999999999998e1_f64 * t41696 + 0.11554466666666666666e1_f64 * t41703 - 0.9628722222222222222e0_f64 * t41716 + 0.34663399999999999999e1_f64 * t41723 - 0.38514888888888888888e0_f64 * t41731 + 0.38514888888888888888e0_f64 * t41737 - 0.11554466666666666666e1_f64 * t41741 + 0.1056393e1_f64 * t43236 * t2417 - 0.469508e0_f64 * t10308 * t9600;
    (t43236, t43241)
}
