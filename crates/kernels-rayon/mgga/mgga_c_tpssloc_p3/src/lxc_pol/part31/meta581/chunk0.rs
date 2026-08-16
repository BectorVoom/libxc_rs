//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1820/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1820(t26411: f64, t6914: f64, t22704: f64, t22705: f64, t5345: f64, t22690: f64, t552: f64, t26447: f64, t90607: f64, t22751: f64, t26397: f64, t22892: f64, t22893: f64, t26396: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90759 = t6914 * t26411;
    let t90781 = t22704 * t22705 * t5345;
    let t90787 = t22690 * t552;
    let t90789 = t90607 * t90787 * t26447;
    let t90791 = t22751 * t26397;
    let t90794 = t22892 * t22893 * t26396;
    (t90759, t90781, t90787, t90789, t90791, t90794)
}
