//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 916/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk916(t238: f64, t17876: f64, t17931: f64, t17992: f64, t18136: f64, t676: f64, t27: f64, t89: f64, t375: f64, t4935: f64, t5054: f64, t2371: f64, t5053: f64) -> (f64, f64, f64, f64, f64) {
    let t239 = 0.1e-59_f64 < t238;
    let t18139 = piecewise3(t239, t17876 + t17931 + t17992 + t18136, 0.0_f64);
    let t18140 = t676 * t18139;
    let t18142 = t89 * t27 * t18140;
    let t18145 = t89 * t375 * t4935;
    let t18148 = t89 * t375 * t5054;
    let t18150 = t2371 * t5053;
    (t18139, t18142, t18145, t18148, t18150)
}
