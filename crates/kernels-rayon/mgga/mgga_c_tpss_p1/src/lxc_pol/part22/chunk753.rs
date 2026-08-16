//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 753/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk753(t2724: f64, t948: f64, t3932: f64, t3931: f64, t1465: f64, t2675: f64, t242: f64, t946: f64, t837: f64, t2741: f64, t1461: f64, t1467: f64, t2665: f64, t2670: f64, t2682: f64, t2685: f64, t2690: f64, t2722: f64, t2740: f64, t3917: f64, t3920: f64, t3924: f64, t3928: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3933 = t2724 * t948;
    let t3934 = t3932 * t3933;
    let t3935 = t3931 * t3934;
    let t3940 = t2675 * t1465;
    let t3941 = t242 * t3940;
    let t3942 = t946 * t3941;
    let t3944 = t1465 * t837;
    let t3945 = t2741 * t3944;
    let t3948 = -t2665 / 108.0_f64 - t2670 + t2690 / 864.0_f64 - t2685 * t1461 / 108.0_f64 + t3917 / 864.0_f64 + t925 * t3920 / 216.0_f64 - t925 * t3924 / 144.0_f64 + t925 * t3928 / 288.0_f64 + t2722 * t3935 / 1536.0_f64 - t2682 * t1467 / 576.0_f64 + t3942 / 4608.0_f64 + t2740 * t3945 / 4608.0_f64;
    (t3933, t3934, t3935, t3941, t3944, t3945, t3948)
}
