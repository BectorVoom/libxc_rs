//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1420/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1420(t1992: f64, t550: f64, t6976: f64, t75026: f64, t1985: f64, t1998: f64, t20601: f64, t214: f64, t74967: f64, t74937: f64, t22685: f64, t26395: f64, t6330: f64, t6637: f64) -> (f64, f64, f64, f64, f64) {
    let t107397 = t1992 * t6976 * t75026 * t550;
    let t107402 = t1985 * t214 * t1998 * t20601;
    let t107406 = t1992 * t6976 * t74967 * t550;
    let t107413 = t1992 * t6976 * t74937 * t550;
    let t107417 = t22685 * t6637 * t26395 * t6330;
    (t107397, t107402, t107406, t107413, t107417)
}
