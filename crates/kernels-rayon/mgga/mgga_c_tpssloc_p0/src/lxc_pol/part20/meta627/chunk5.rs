//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2271/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2271(t13084: f64, t13258: f64, t13353: f64, t9638: f64, t41466: f64, t820: f64, t13176: f64, t2642: f64, t10024: f64, t1500: f64, t13293: f64, t9573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47027 = t13258 * t13084;
    let t47037 = t9638 * t13353;
    let t47039 = t41466 * t820;
    let t47044 = t13176 * t2642;
    let t47047 = t1500 * t10024;
    let t47049 = t9573 * t13293;
    (t47027, t47037, t47039, t47044, t47047, t47049)
}
