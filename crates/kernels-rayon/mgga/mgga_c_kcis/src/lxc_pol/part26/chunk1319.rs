//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1319/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1319(t20998: f64, t4160: f64, t94425: f64, t21003: f64, t98530: f64, t21792: f64, t2243: f64, t303: f64, t1928: f64, t2050: f64, t1394: f64, t7924: f64) -> (f64, f64, f64, f64) {
    let t102626 = t4160 * t94425 * t20998;
    let t102629 = t4160 * t98530 * t21003;
    let t102632 = t303 * t21792 * t2243;
    let t102634 = t2050 * t1928;
    let t102636 = t1394 * t102634 * t7924;
    (t102626, t102629, t102632, t102636)
}
