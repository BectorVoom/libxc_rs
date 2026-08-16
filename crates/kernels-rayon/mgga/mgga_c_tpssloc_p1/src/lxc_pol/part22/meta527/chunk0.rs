//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1997/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1997(t1068: f64, t1637: f64, t1458: f64, t649: f64, t4072: f64, t88: f64, t89: f64, t254: f64, t563: f64, t1351: f64, t16311: f64, t16306: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25845 = t1637 * t1068;
    let t26114 = t649 * t1458;
    let t26117 = t88 * t4072;
    let t26179 = t89 * t4072;
    let t26224 = t563 * t254;
    let t26318 = t16311 * t1351;
    let t26322 = t16306 * t550;
    (t25845, t26114, t26117, t26179, t26224, t26318, t26322)
}
