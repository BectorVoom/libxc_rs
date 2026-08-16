//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2012/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2012(t254: f64, t563: f64, t1351: f64, t1834: f64, t492: f64, t64: f64, t9365: f64, t1444: f64, t659: f64, t1449: f64, t662: f64, t20: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26224 = t563 * t254;
    let t26409 = t1834 * t1351;
    let t27784 = t492 * t254;
    let t29903 = t64 * t9365;
    let t30171 = t1444 * t659;
    let t30307 = t1449 * t662;
    let t32253 = 1.0_f64 / t60 / t20;
    (t26224, t26409, t27784, t29903, t30171, t30307, t32253)
}
