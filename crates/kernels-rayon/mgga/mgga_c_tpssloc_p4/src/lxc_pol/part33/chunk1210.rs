//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1210/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1210(t2239: f64, t5385: f64, t1597: f64, t976: f64, t3131: f64, t5866: f64, t111: f64, t20292: f64, t21038: f64, t225: f64, t21061: f64, t21036: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55921 = t5385 * t2239;
    let t61066 = t976 * t1597;
    let t62840 = t5866 * t3131;
    let t67001 = t20292 * t111;
    let t67305 = t21038 * t225;
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    (t55921, t61066, t62840, t67001, t67305, t67339, t67344)
}
