//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1269/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1269(t20446: f64, t219: f64, t6338: f64, t18000: f64, t6342: f64, t818: f64, t18770: f64, t19748: f64, t1396: f64, t17993: f64, t18006: f64, t1809: f64, t18753: f64, t19734: f64, t19736: f64, t253: f64, t3699: f64, t3722: f64, t5571: f64, t5834: f64, t5838: f64, t5843: f64, t5846: f64, t6135: f64, t6343: f64, t819: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t20447 = param_beta * t20446;
    let t20449 = t6338 * t219;
    let t20463 = t18000 * t6342 * t818;
    let t20466 = t18770 * t19748;
    let t20469 = -t1396 * t18753 + 2.0_f64 * t17993 * t6343 - 2.0_f64 * t18006 * t20466 - t1809 * t19734 + 2.0_f64 * t19736 * t5838 + t19736 * t5843 + t20447 * t253 - t20449 * t819 - 6.0_f64 * t20463 * t5571 + 2.0_f64 * t3699 * t5834 - t3722 * t5834 - t5846 * t6135;
    (t20447, t20449, t20463, t20466, t20469)
}
