//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2041/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2041(t101479: f64, t102719: f64, t13426: f64, t13429: f64, t14310: f64, t1502: f64, t1519: f64, t2056: f64, t2089: f64, t2093: f64, t2331: f64, t25082: f64, t25188: f64, t26162: f64, t26405: f64, t26415: f64, t26674: f64, t28167: f64, t28286: f64, t28653: f64, t28658: f64, t3813: f64, t4248: f64, t4257: f64, t49686: f64, t5787: f64, t73394: f64, t73488: f64, t7367: f64, t7484: f64, t75667: f64, t7898: f64, t7969: f64, t8079: f64, t8111: f64, t98436: f64) -> f64 {
    let t103956 = 3.0_f64 * t25188 * t8079 - 2.0_f64 * t13429 * t2089 + 6.0_f64 * t25082 * t28286 * t73488 - t25188 * t8111 + 6.0_f64 * t7898 * t26162 - 6.0_f64 * t25082 * t26405 * t73394 - 12.0_f64 * t28167 * t26405 * t101479 - 2.0_f64 * t102719 * t1519 - 4.0_f64 * t28658 * t4257 - 3.0_f64 * t25082 * t26405 * t98436 - t7969 * t3813 - t1502 * t26674 - 4.0_f64 * t28653 * t2331 - 2.0_f64 * t4248 * t26415 + 2.0_f64 * t7484 * t5787 + t2093 * t14310 - 2.0_f64 * t49686 * t2056 - 4.0_f64 * t75667 * t2056 - 4.0_f64 * t13426 * t7367;
    t103956
}
