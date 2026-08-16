//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1229/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1229(t7756: f64, t8690: f64, t2165: f64, t7467: f64, t652: f64, t4028: f64, t8675: f64, t7458: f64, t1873: f64, t8103: f64, t1458: f64, t8682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33725 = t8690 * t7756;
    let t33726 = t2165 * t7467;
    let t33727 = t652 * t33726;
    let t33731 = t4028 * t8675;
    let t33733 = t7458 * t8675;
    let t33735 = t8103 * t1873;
    let t33736 = t652 * t33735;
    let t33740 = t8682 * t1458;
    (t33725, t33726, t33727, t33731, t33733, t33735, t33736, t33740)
}
