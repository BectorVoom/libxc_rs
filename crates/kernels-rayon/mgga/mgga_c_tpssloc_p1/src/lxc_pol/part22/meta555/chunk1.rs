//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2056/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2056(t2691: f64, t812: f64, t815: f64, t10024: f64, t809: f64, t238: f64, t244: f64, t248: f64, t40445: f64, t9525: f64, t9577: f64, t116: f64) -> (f64, f64, f64, f64, f64) {
    let t41115 = t812 * t815 * t2691;
    let t41130 = t809 * t10024;
    let t41139 = 13685.0_f64 / 31104.0_f64 * t238 * t40445 * t244 * t248;
    let t41144 = t9577 * t9525;
    let t41146 = t244 * t116;
    (t41115, t41130, t41139, t41144, t41146)
}
