//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1318/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1318(t2411: f64, t2888: f64, t2099: f64, t3235: f64, t8419: f64, t8410: f64, t8414: f64, t23213: f64, t3206: f64, t8255: f64, t1220: f64, t6433: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23278 = t2888 * t2411;
    let t23286 = t3235 * t2099 * t8419;
    let t23296 = t3235 * t2099 * t8410;
    let t23299 = t3235 * t2099 * t8414;
    let t23311 = t3206 * t23213 * t8255;
    let t23313 = t1220 * t6433;
    (t23278, t23286, t23296, t23299, t23311, t23313)
}
