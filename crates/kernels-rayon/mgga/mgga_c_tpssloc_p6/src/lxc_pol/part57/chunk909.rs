//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 909/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk909(t241: f64, t812: f64, t814: f64, t835: f64, t23094: f64, t30703: f64, t23103: f64, t794: f64, t8339: f64, t226: f64, t235: f64, t2690: f64, t8344: f64) -> (f64, f64, f64, f64) {
    let t112802 = t812 * t814 * t835 * t241;
    let t112834 = t23094 * t30703;
    let t112840 = t23103 * t794 * t8339;
    let t112850 = t226 * t235 * t2690 * t8344;
    (t112802, t112834, t112840, t112850)
}
