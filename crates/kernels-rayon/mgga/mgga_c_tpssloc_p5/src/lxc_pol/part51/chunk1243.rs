//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1243/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1243(t31758: f64, t7687: f64, t1983: f64, t5161: f64, t8640: f64, t7688: f64, t8607: f64, t2075: f64, t7467: f64, t652: f64, t1458: f64, t8595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33603 = t31758 * t7687;
    let t33605 = 3.0_f64 * t1983 * t33603;
    let t33610 = t8640 * t5161;
    let t33611 = t1983 * t33610;
    let t33615 = 3.0_f64 * t8607 * t7688;
    let t33617 = t2075 * t7467;
    let t33619 = 2.0_f64 * t652 * t33617;
    let t33620 = t8595 * t1458;
    (t33603, t33605, t33610, t33611, t33615, t33617, t33619, t33620)
}
