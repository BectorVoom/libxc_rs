//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1594/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1594(t23230: f64, t225: f64, t7072: f64, t7085: f64, t23251: f64, t23261: f64, t2752: f64, t7109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24291 = 0.16449340668482264365e-1_f64 * t23230;
    let t24297 = t7072 * t225;
    let t24305 = t7085 * t225;
    let t24318 = 0.52089578783527170489e-1_f64 * t23251;
    let t24321 = 0.12793931631041761173e0_f64 * t23261;
    let t24339 = t7109 * t2752;
    (t24291, t24297, t24305, t24318, t24321, t24339)
}
