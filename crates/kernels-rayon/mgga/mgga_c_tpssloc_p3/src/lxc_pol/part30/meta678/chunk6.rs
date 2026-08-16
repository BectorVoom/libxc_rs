//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2126/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2126(t28827: f64, t6876: f64, t7684: f64, t8944: f64, t26164: f64, t24995: f64, t75203: f64, t8643: f64, t34999: f64, t5308: f64, t28813: f64, t19577: f64, t22574: f64, t33136: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96796 = 6.0_f64 * t6876 * t28827;
    let t96797 = t7684 * t8944;
    let t96799 = 4.0_f64 * t96797 * t26164;
    let t96802 = 6.0_f64 * t24995 * t8643 * t75203;
    let t96805 = 12.0_f64 * t24995 * t34999 * t5308;
    let t96807 = 2.0_f64 * t6876 * t28813;
    let t96813 = 6.0_f64 * t22574 * t33136 * t19577;
    (t96796, t96799, t96802, t96805, t96807, t96813)
}
