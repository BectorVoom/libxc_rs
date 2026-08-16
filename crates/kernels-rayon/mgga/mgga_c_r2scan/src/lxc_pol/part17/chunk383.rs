//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 383/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk383(t1604: f64, t1607: f64, t774: f64, t784: f64, t783: f64, t788: f64, t162: f64, t38: f64) -> (f64, f64, f64, f64) {
    let t1608 = t1604 * t1607;
    let t1610 = t774 * t784;
    let t1612 = t783 * t1610 * t788;
    let t1614 = t162 * t38;
    let t1615 = 1.0_f64 / t1614;
    (t1608, t1610, t1612, t1615)
}
