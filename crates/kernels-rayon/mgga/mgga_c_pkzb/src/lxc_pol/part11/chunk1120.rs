//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1120/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1120(t2411: f64, t2888: f64, t154: f64, t3026: f64, t385: f64, t6446: f64, t1220: f64, t6448: f64, t1167: f64, t19023: f64, t3214: f64, t6467: f64) -> (f64, f64, f64, f64, f64) {
    let t23278 = t2888 * t2411;
    let t23317 = t385 * t154 * t6446 * t3026;
    let t23318 = t23317 / 144.0_f64;
    let t23331 = t1220 * t6448;
    let t23332 = t23331 / 54.0_f64;
    let t23338 = t385 * t154 * t19023 * t1167;
    let t23340 = t3214 * t6467;
    (t23278, t23318, t23332, t23338, t23340)
}
