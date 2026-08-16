//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 781/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk781(t2020: f64, t7685: f64, t1390: f64, t1799: f64, t6878: f64, t1983: f64, t6890: f64) -> (f64, f64, f64, f64, f64) {
    let t7686 = t7685 * t2020;
    let t7687 = t1390 * t1799;
    let t7688 = t6878 * t7687;
    let t7690 = 3.0_f64 * t1983 * t7688;
    let t7691 = t6890 * t1799;
    (t7686, t7687, t7688, t7690, t7691)
}
