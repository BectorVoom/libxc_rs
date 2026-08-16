//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 830/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk830(t44294: f64, t6508: f64, t1358: f64, t6507: f64, t2339: f64, t35918: f64, t42581: f64, t10231: f64, t1365: f64, t42529: f64, t44258: f64, t44262: f64, t44263: f64, t44264: f64, t44267: f64, t44269: f64, t44278: f64, t44281: f64, t44284: f64, t44288: f64, t44292: f64, t44293: f64, t7888: f64) -> (f64, f64) {
    let t44295 = t6508 * t44294;
    let t44298 = 0.63233348079280332442e-2_f64 * t1358 * t6507 * t44295;
    let t44301 = 0.22131671827748116354e-1_f64 * t1358 * t35918 * t2339;
    let t44302 = 0.18970004423784099733e-1_f64 * t42581;
    let t44303 = -t44258 + 0.47425011059460249332e-2_f64 * t42529 + t44262 + t44263 - t44264 - t44267 + 0.31616674039640166221e-2_f64 * t1358 * t1365 * t44269 + 0.18970004423784099732e-1_f64 * t1358 * t7888 * t10231 - t44278 + t44281 - t44284 + t44288 - t44292 - t44293 - t44298 + t44301 + t44302;
    (t44295, t44303)
}
