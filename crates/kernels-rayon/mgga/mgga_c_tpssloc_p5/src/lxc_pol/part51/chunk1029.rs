//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1029/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1029(t25605: f64, t25631: f64, t25672: f64, t25703: f64, t383: f64, t4673: f64, t7619: f64, t1598: f64, t984: f64, t23478: f64, t6785: f64, t4347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25705 = t25605 + t25631 + t25672 + t25703;
    let t25706 = t383 * t25705;
    let t25708 = t7619 * t4673;
    let t25712 = t1598 * t984;
    let t25713 = t23478 * t6785;
    let t25714 = t25712 * t25713;
    let t25717 = t6785 * t4347;
    (t25705, t25706, t25708, t25712, t25714, t25717)
}
