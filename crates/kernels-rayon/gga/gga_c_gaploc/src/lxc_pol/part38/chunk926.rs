//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 926/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk926(t13625: f64, t2684: f64, t7354: f64, t13626: f64, t2013: f64, t11724: f64, t2464: f64, t2465: f64, t825: f64, t43912: f64, t2639: f64, t3601: f64, t7284: f64, t787: f64) -> (f64, f64, f64, f64, f64) {
    let t45723 = t2684 * t7354 * t13625;
    let t45725 = t2013 * t13626;
    let t45729 = t825 * t2464 * t2465 * t11724;
    let t45731 = 0.11916829983950142223e0_f64 * t43912;
    let t45735 = 0.53625734927775640005e1_f64 * t787 * t7284 * t3601 * t2639;
    (t45723, t45725, t45729, t45731, t45735)
}
