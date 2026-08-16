//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 511/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk511(t2609: f64, t557: f64, t1501: f64, t1510: f64, t1520: f64, t1530: f64, t1534: f64, t1544: f64, t1547: f64, t1553: f64, t2535: f64, t2559: f64, t2606: f64, t2608: f64) -> (f64, f64, f64) {
    let t2610 = t2609 * t557;
    let t2611 = 0.5848223622634646207e0_f64 * t2610;
    let t2612 = -t1501 - t1510 - t2535 - t1520 + t1530 + t1534 + t2559 + t2606 + t2608 + t1544 - t1547 - t2611 - t1553;
    (t2610, t2611, t2612)
}
