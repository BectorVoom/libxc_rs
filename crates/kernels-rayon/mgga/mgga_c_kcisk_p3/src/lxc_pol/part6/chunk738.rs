//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 738/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk738(t12825: f64, t41: f64, t12829: f64, t451: f64, t13329: f64, t492: f64, t1555: f64, t524: f64, t4349: f64, t544: f64, t13399: f64, t13064: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14496 = t41 * t12825;
    let t14497 = t451 * t12829;
    let t14545 = t13329 * t492;
    let t14607 = t1555 * t1555;
    let t14608 = 1.0_f64 / t14607;
    let t14609 = t524 * t14608;
    let t14612 = 1.0_f64 / t4349 / t544;
    let t14665 = 0.51588271604938271604e-3_f64 * t13399;
    let t14736 = t325 * t13064;
    (t14496, t14497, t14545, t14609, t14612, t14665, t14736)
}
