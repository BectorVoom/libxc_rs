//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 662/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk662(t11613: f64, t313: f64, t3650: f64, t773: f64, t1645: f64, t2963: f64, t11777: f64, t11781: f64, t11785: f64, t11788: f64, t11792: f64, t11795: f64, t11798: f64, t1966: f64, t1991: f64, t2087: f64, t2639: f64, t3025: f64, t5640: f64, t5974: f64, t813: f64, t833: f64, t9858: f64, t9873: f64) -> (f64, f64) {
    let t11801 = t313 * t11613;
    let t11804 = t773 * t3650;
    let t11807 = t1645 * t2963;
    let t11811 = 0.51123901271894332902e0_f64 * t1991 * t11777 + 0.15337170381568299871e1_f64 * t5640 * t11781 - 0.51123901271894332902e0_f64 * t1966 * t11785 - 0.62115540045351614476e2_f64 * t2087 * t11788 - 0.46011511144704899612e1_f64 * t813 * t11792 + 0.11502877786176224903e2_f64 * t833 * t11795 - 0.10725146985555128001e1_f64 * t11798 * t2639 + 0.42900587942220512003e1_f64 * t11801 * t9858 + 0.10725146985555128001e1_f64 * t11804 * t5974 - 0.21450293971110256002e1_f64 * t3025 * t11807 - 0.31952438294933958063e-1_f64 * t9873;
    (t11801, t11811)
}
