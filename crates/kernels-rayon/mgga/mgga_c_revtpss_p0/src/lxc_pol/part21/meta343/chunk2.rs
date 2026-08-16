//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1672/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1672(t11291: f64, t11293: f64, t11296: f64, t11303: f64, t11382: f64, t11390: f64, t11521: f64, t11525: f64, t11530: f64, t11533: f64, t11547: f64, t11548: f64, t11551: f64, t11554: f64, t11557: f64, t11572: f64, t11585: f64, t2945: f64, t2968: f64, t2987: f64, t2989: f64, t3012: f64, t311: f64) -> f64 {
    let t11588 = -0.35089341735807877242e1_f64 * t2987 * t11521 + 0.51947577317044391277e2_f64 * t3012 * t11525 + t11530 - t11533 + t11547 - t11291 - t11293 - t11296 + t11303 - t11382 - t11390 - 6.0_f64 * t11548 * t2945 + 6.0_f64 * t2968 * t11551 - 0.35089341735807877242e1_f64 * t11554 * t2989 + 0.35089341735807877242e1_f64 * t3012 * t11557 - 0.19751673498613801407e-1_f64 * t11572 - 0.310907e-1_f64 * t11585 * t311;
    t11588
}
