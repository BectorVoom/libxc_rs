//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 806/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk806(t2862: f64, t7584: f64, t882: f64, t296: f64, t34019: f64, t34017: f64, t34015: f64, t34054: f64, t7611: f64, t840: f64, t319: f64, t33953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34130 = t2862 * t882 * t7584;
    let t34133 = t296 * t34019;
    let t34136 = t296 * t34017;
    let t34139 = t296 * t34015;
    let t34142 = t296 * t34054;
    let t34146 = t840 * t882 * t7611;
    let t34150 = t840 * t319 * t33953;
    (t34130, t34133, t34136, t34139, t34142, t34146, t34150)
}
