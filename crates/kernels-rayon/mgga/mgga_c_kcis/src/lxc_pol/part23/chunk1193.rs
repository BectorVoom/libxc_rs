//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1193/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1193(t12344: f64, t2247: f64, t1598: f64, t251: f64, t40512: f64, t40515: f64, t617: f64, t40484: f64, t18210: f64, t27597: f64, t27595: f64, t27647: f64, t7978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94833 = t2247 * t12344;
    let t94861 = t40512 * t251 * t1598;
    let t94862 = t617 * t40515;
    let t94901 = t40484 * t251 * t1598;
    let t94904 = t18210 * t27597;
    let t94905 = t27595 * t94904;
    let t94914 = t7978 * t18210 * t27647;
    (t94833, t94861, t94862, t94901, t94904, t94905, t94914)
}
