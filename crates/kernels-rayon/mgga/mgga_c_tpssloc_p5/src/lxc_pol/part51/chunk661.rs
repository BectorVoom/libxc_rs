//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 661/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk661(t1385: f64, t1842: f64, t3887: f64, t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64, t5250: f64, t1825: f64, t3901: f64, t1380: f64, t5287: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5325 = t1842 * t1385;
    let t5326 = t3887 * t5325;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5335 = t562 * t1824;
    let t5336 = t5335 * t5250;
    let t5339 = t3901 * t1825;
    let t5341 = t1380 * t5287;
    (t5325, t5326, t5334, t5335, t5336, t5339, t5341)
}
