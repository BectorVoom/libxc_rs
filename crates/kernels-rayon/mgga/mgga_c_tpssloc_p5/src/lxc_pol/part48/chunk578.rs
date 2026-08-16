//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 578/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk578(t240: f64, t6943: f64, t1336: f64, t1354: f64, t1358: f64, t2003: f64, t552: f64, t59: f64, t1369: f64, t6915: f64, t6917: f64, t6922: f64, t6929: f64, t6935: f64, t6938: f64, t6941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6944 = t6943 * t240;
    let t6945 = t1336 * t6944;
    let t6946 = t6945 * t1354;
    let t6948 = t2003 * t1358;
    let t6949 = 7.0_f64 / 2304.0_f64 * t6948;
    let t6950 = t552 * t59;
    let t6951 = t6950 * t240;
    let t6952 = t1336 * t6951;
    let t6953 = t6952 * t1369;
    let t6955 = -t6915 - t6917 / 48.0_f64 - t6922 - 0.12111826828242117256e-2_f64 * t6929 - t6935 - 0.20186378047070195427e-3_f64 * t6938 + t6941 / 1536.0_f64 - t6946 / 1536.0_f64 - t6949 - t6953 / 384.0_f64;
    (t6944, t6945, t6946, t6948, t6950, t6951, t6952, t6953, t6955)
}
