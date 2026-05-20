//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2041/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2041<F: Float>(t101479: F, t102719: F, t13426: F, t13429: F, t14310: F, t1502: F, t1519: F, t2056: F, t2089: F, t2093: F, t2331: F, t25082: F, t25188: F, t26162: F, t26405: F, t26415: F, t26674: F, t28167: F, t28286: F, t28653: F, t28658: F, t3813: F, t4248: F, t4257: F, t49686: F, t5787: F, t73394: F, t73488: F, t7367: F, t7484: F, t75667: F, t7898: F, t7969: F, t8079: F, t8111: F, t98436: F) -> F {
    let t103956 = F::new(3.0) * t25188 * t8079 - F::new(2.0) * t13429 * t2089 + F::new(6.0) * t25082 * t28286 * t73488 - t25188 * t8111 + F::new(6.0) * t7898 * t26162 - F::new(6.0) * t25082 * t26405 * t73394 - F::new(12.0) * t28167 * t26405 * t101479 - F::new(2.0) * t102719 * t1519 - F::new(4.0) * t28658 * t4257 - F::new(3.0) * t25082 * t26405 * t98436 - t7969 * t3813 - t1502 * t26674 - F::new(4.0) * t28653 * t2331 - F::new(2.0) * t4248 * t26415 + F::new(2.0) * t7484 * t5787 + t2093 * t14310 - F::new(2.0) * t49686 * t2056 - F::new(4.0) * t75667 * t2056 - F::new(4.0) * t13426 * t7367;
    t103956
}
