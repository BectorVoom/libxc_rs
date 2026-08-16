//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 950/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk950(t2042: f64, t4265: f64, t1533: f64, t1543: f64, t5935: f64, t2061: f64, t4297: f64, t1546: f64, t2060: f64, t577: f64, t1467: f64, t4294: f64) -> (f64, f64, f64, f64, f64) {
    let t17496 = t2042 * t4265;
    let t17497 = t1533 * t17496;
    let t17499 = t1543 * t5935;
    let t17501 = t2061 * t4297;
    let t17502 = t1546 * t17501;
    let t17504 = t577 * t2060;
    let t17505 = t1467 * t17504;
    let t17506 = t17505 * t4294;
    (t17497, t17499, t17501, t17502, t17506)
}
