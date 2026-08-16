//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1107/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1107(t30788: f64, t7553: f64, t6705: f64, t7624: f64, t6704: f64, t30854: f64, t7565: f64, t1599: f64, t8376: f64, t1603: f64, t8391: f64, t32961: f64, t349: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32987 = t30788 * t7553;
    let t32992 = t6705 * t7624;
    let t32993 = t6704 * t32992;
    let t32998 = t30854 * t7565;
    let t33001 = t1599 * t8376;
    let t33005 = t1603 * t8391;
    let t33007 = t349 * t32961;
    (t32987, t32992, t32993, t32998, t33001, t33005, t33007)
}
