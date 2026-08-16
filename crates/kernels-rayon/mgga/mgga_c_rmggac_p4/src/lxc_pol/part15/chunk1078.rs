//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1078/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1078(t2289: f64, t9090: f64, t1356: f64, t36701: f64, t41829: f64, t41883: f64, t41885: f64, t46050: f64, t46758: f64, t47629: f64, t47634: f64, t47639: f64, t47644: f64, t47646: f64, t47653: f64, t47663: f64, t47667: f64, t4965: f64, t4985: f64, t5928: f64, t739: f64, t8804: f64, t8824: f64, t9867: f64) -> f64 {
    let t47669 = t9090 * t2289;
    let t47671 = 0.53205749866622299248e-5_f64 * t47629 + 0.85129199786595678796e-5_f64 * t47634 - 0.12769379967989351819e-4_f64 * t47639 + 0.12769379967989351819e-4_f64 * t47644 - t41829 - 0.59590439850616975155e-4_f64 * t47646 + 0.79828278012425390428e-1_f64 * t5928 * t8804 + 0.51077519871957407276e-4_f64 * t47653 + 0.79828278012425390428e-1_f64 * t4965 * t9867 + 0.11974241701863808564e0_f64 * t4985 * t8824 - 0.11974241701863808564e0_f64 * t1356 * t46050 - 0.59871208509319042821e-1_f64 * t739 * t46758 - 0.59590439850616975155e-4_f64 * t47663 - 0.42564599893297839398e-5_f64 * t47667 - t36701 + t41883 + t41885 + 0.59590439850616975155e-4_f64 * t47669;
    t47671
}
