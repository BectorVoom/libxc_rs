//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1197/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1197(t18574: f64, t5142: f64, t18677: f64, t18672: f64, t5134: f64, t1018: f64, t1745: f64, t4581: f64, t2840: f64, t4567: f64, t19536: f64, t304: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19986 = t5142 * t18574;
    let t19989 = t5142 * t18677;
    let t19992 = t5134 * t18672;
    let t19995 = t1018 * t1745;
    let t19996 = t19995 * t4581;
    let t19999 = t2840 * t1745;
    let t20000 = t19999 * t4567;
    let t20003 = t304 * t19536;
    (t19986, t19989, t19992, t19996, t20000, t20003)
}
