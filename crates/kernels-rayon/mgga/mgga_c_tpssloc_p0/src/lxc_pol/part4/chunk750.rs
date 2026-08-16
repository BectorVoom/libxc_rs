//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 750/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk750(t5257: f64, t5317: f64, t539: f64, t1835: f64, t225: f64, t1385: f64, t1842: f64, t3887: f64, t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5318 = t5257 + t5317;
    let t5319 = t539 * t5318;
    let t5321 = t1835 * t225;
    let t5325 = t1842 * t1385;
    let t5326 = t3887 * t5325;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5335 = t562 * t1824;
    (t5318, t5319, t5321, t5326, t5333, t5334, t5335)
}
