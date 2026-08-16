//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 930/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk930(t242: f64, t837: f64, t8951: f64, t967: f64, t2655: f64, t943: f64, t938: f64, t941: f64, t357: f64, t339: f64, t349: f64, t2677: f64, t2682: f64) -> (f64, f64, f64, f64) {
    let t8953 = t242 * t8951 * t837;
    let t8954 = t967 * t8953;
    let t8956 = t2655 * t943;
    let t8958 = t938 * t941 * t8956;
    let t8961 = t2655 * t357;
    let t8963 = t339 * t349 * t8961;
    let t8966 = t2682 * t2677;
    (t8954, t8958, t8963, t8966)
}
