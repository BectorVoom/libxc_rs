//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 761/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk761(t2013: f64, t35704: f64, t7338: f64, t7491: f64, t20963: f64, t82: f64, t1338: f64, t2010: f64, t7352: f64, t31: f64, t34790: f64, t7349: f64) -> (f64, f64, f64, f64) {
    let t35705 = t35704 * t2013;
    let t35707 = t7491 * t7338;
    let t35709 = t20963 * t82;
    let t35712 = t2010 * t35709 * t7352 * t1338;
    let t35713 = 0.91462949374725084942e-3_f64 * t35712;
    let t35716 = t7349 * t35709 * t34790 * t31;
    (t35705, t35707, t35713, t35716)
}
