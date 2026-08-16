//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2112/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2112(t2403: f64, t2830: f64, t10317: f64, t699: f64, t909: f64, t9709: f64, t10310: f64, t2833: f64, t2827: f64, t10322: f64, t10306: f64, t10213: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41831 = t2403 * t2830;
    let t41833 = t699 * t10317;
    let t41863 = t9709 * t909;
    let t41865 = t699 * t10310;
    let t41870 = t2403 * t2833;
    let t41872 = t2403 * t2827;
    let t41874 = t699 * t10322;
    let t41876 = t699 * t10306;
    let t41880 = t241 * t10213;
    (t41831, t41833, t41863, t41865, t41870, t41872, t41874, t41876, t41880)
}
