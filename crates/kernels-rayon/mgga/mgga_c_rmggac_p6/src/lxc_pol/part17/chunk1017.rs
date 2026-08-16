//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1017/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1017(t2060: f64, t6463: f64, t305: f64, t27101: f64, t46533: f64, t25854: f64, t46537: f64, t10189: f64, t321: f64, t333: f64, t352: f64, t36284: f64, t36286: f64, t41524: f64, t46558: f64, t46582: f64, t46592: f64, t46626: f64, t4669: f64, t5148: f64, t8940: f64) -> (f64, f64, f64) {
    let t46736 = t2060 * t6463;
    let t46737 = t305 * t46736;
    let t46748 = t27101 * t46533;
    let t46750 = t25854 * t46537;
    let t46758 = t10189 * t321;
    let t46763 = -0.14967802127329760705e-1_f64 * t46737 - 0.35922725105591425692e0_f64 * t4669 * t46582 * t333 - 0.17961362552795712846e0_f64 * t4669 * t46592 * t321 + 0.23948483403727617128e0_f64 * t8940 * t46626 * t352 + 0.5987120850931904282e-1_f64 * t46748 - 0.8980681276397856423e-1_f64 * t46750 - 0.17961362552795712846e0_f64 * t4669 * t46558 * t333 - 0.11974241701863808564e0_f64 * t5148 * t46558 * t352 + 0.59871208509319042821e-1_f64 * t305 * t46758 + 0.2927036860455597649e0_f64 * t36284 - 0.43905552906833964735e0_f64 * t36286 + t41524;
    (t46736, t46758, t46763)
}
