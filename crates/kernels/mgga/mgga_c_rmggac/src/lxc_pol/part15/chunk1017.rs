//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1017/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1017<F: Float>(t2060: F, t6463: F, t305: F, t27101: F, t46533: F, t25854: F, t46537: F, t10189: F, t321: F, t333: F, t352: F, t36284: F, t36286: F, t41524: F, t46558: F, t46582: F, t46592: F, t46626: F, t4669: F, t5148: F, t8940: F) -> (F, F, F) {
    let t46736 = t2060 * t6463;
    let t46737 = t305 * t46736;
    let t46748 = t27101 * t46533;
    let t46750 = t25854 * t46537;
    let t46758 = t10189 * t321;
    let t46763 = -F::new(0.14967802127329760705e-1) * t46737 - F::new(0.35922725105591425692e0) * t4669 * t46582 * t333 - F::new(0.17961362552795712846e0) * t4669 * t46592 * t321 + F::new(0.23948483403727617128e0) * t8940 * t46626 * t352 + F::new(0.5987120850931904282e-1) * t46748 - F::new(0.8980681276397856423e-1) * t46750 - F::new(0.17961362552795712846e0) * t4669 * t46558 * t333 - F::new(0.11974241701863808564e0) * t5148 * t46558 * t352 + F::new(0.59871208509319042821e-1) * t305 * t46758 + F::new(0.2927036860455597649e0) * t36284 - F::new(0.43905552906833964735e0) * t36286 + t41524;
    (t46736, t46758, t46763)
}
