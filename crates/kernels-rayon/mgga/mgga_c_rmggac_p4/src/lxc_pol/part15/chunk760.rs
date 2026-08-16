//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 760/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk760(t2019: f64, t271: f64, t3118: f64, t641: f64, t7491: f64, t7927: f64, t20: f64, t2018: f64, t2021: f64, t4710: f64, t261: f64, t7581: f64) -> (f64, f64, f64, f64) {
    let t35696 = t2019 * t3118 * t271 * t641;
    let t35697 = 0.44715219694310041527e-2_f64 * t35696;
    let t35698 = t7491 * t7927;
    let t35699 = 0.24390119833260022651e-2_f64 * t35698;
    let t35702 = t4710 * t20 * t2018 * t2021;
    let t35703 = 0.91462949374725084942e-3_f64 * t35702;
    let t35704 = t261 * t7581;
    (t35697, t35699, t35703, t35704)
}
