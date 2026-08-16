//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2135/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2135(t24995: f64, t34999: f64, t5308: f64, t28813: f64, t6876: f64, t19577: f64, t22574: f64, t33136: f64, t19451: f64, t6535: f64, t28830: f64, t31035: f64) -> (f64, f64, f64, f64, f64) {
    let t96805 = 12.0_f64 * t24995 * t34999 * t5308;
    let t96807 = 2.0_f64 * t6876 * t28813;
    let t96813 = 6.0_f64 * t22574 * t33136 * t19577;
    let t96815 = 2.0_f64 * t19451 * t6535;
    let t96818 = 6.0_f64 * t22574 * t31035 * t28830;
    (t96805, t96807, t96813, t96815, t96818)
}
