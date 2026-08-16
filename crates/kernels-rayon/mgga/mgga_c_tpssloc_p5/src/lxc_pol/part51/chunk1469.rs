//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1469/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1469(t75795: f64, t8657: f64, t100993: f64, t7769: f64, t24465: f64, t26542: f64, t26545: f64, t112: f64, t33627: f64, t16524: f64, t31817: f64, t115984: f64, t115996: f64, t120807: f64, t120809: f64, t120818: f64, t1458: f64, t23880: f64, t27170: f64, t27281: f64, t5376: f64, t671: f64, t7010: f64) -> f64 {
    let t122800 = 27.0_f64 * t75795 * t8657;
    let t122804 = 27.0_f64 * t100993 * t7769;
    let t122806 = 27.0_f64 * t24465 * t26542;
    let t122808 = 27.0_f64 * t24465 * t26545;
    let t122811 = t33627 * t112;
    let t122817 = 27.0_f64 * t16524 * t31817;
    let t122820 = t122800 + 0.135e2_f64 * t7010 * t27170 + t120807 + t122804 + t122806 + t122808 + 27.0_f64 * t115984 * t5376 + 0.135e2_f64 * t122811 * t671 + t120809 + 27.0_f64 * t23880 * t27281 + t122817 + t120818 + 0.135e2_f64 * t115996 * t1458;
    t122820
}
