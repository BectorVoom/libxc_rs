//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1305/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1305(t22986: f64, t22997: f64, t2647: f64, t6646: f64, t1887: f64, t23069: f64, t22989: f64, t22690: f64, t23153: f64, t23171: f64, t6561: f64, t80741: f64) -> (f64, f64, f64, f64, f64) {
    let t81589 = t22986 * t6646 * t22997 * t2647;
    let t81591 = t23069 * t1887;
    let t81592 = t81591 * t22989;
    let t81595 = t23171 * t22690 * t23153;
    let t81597 = t80741 * t6561;
    (t81589, t81591, t81592, t81595, t81597)
}
