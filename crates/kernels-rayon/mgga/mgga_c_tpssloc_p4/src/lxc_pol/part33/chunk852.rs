//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 852/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk852(t12328: f64, t555: f64, t10027: f64, t541: f64, t1361: f64, t2690: f64, t1336: f64, t241: f64, t67: f64, t6924: f64, t1339: f64, t3788: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12330 = 595.0_f64 / 10368.0_f64 * t555 * t12328;
    let t12335 = 455.0_f64 / 1296.0_f64 * t10027 * t541;
    let t12344 = t1361 * t2690;
    let t12345 = t1336 * t12344;
    let t12351 = t241 * t6924 * t67;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12384 = t3788 * t835;
    (t12330, t12335, t12345, t12351, t12365, t12384)
}
