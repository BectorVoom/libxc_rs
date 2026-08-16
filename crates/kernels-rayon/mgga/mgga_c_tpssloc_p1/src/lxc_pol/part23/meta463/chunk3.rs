//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1357/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1357(t76829: f64, t76865: f64, t76943: f64, t76974: f64, t225: f64, t76634: f64, t76636: f64, t76641: f64, t76643: f64, t76647: f64, t76652: f64, t76654: f64, t76657: f64, t76659: f64, t76661: f64, t76663: f64) -> (f64, f64, f64) {
    let t76976 = t76829 + t76865 + t76943 + t76974;
    let t76977 = t76976 * t225;
    let t76995 = t76634 - t76636 - t76641 + t76643 + t76647 - t76652 - t76654 + t76657 + t76659 + t76661 + t76663;
    (t76976, t76977, t76995)
}
