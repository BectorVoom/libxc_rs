//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 766/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk766(t261: f64, t7581: f64, t2013: f64, t7338: f64, t7491: f64, t20963: f64, t82: f64, t1338: f64, t2010: f64, t7352: f64, t31: f64, t34790: f64, t7349: f64) -> (f64, f64, f64, f64, f64) {
    let t35704 = t261 * t7581;
    let t35705 = t35704 * t2013;
    let t35707 = t7491 * t7338;
    let t35709 = t20963 * t82;
    let t35712 = t2010 * t35709 * t7352 * t1338;
    let t35716 = t7349 * t35709 * t34790 * t31;
    (t35704, t35705, t35707, t35712, t35716)
}
