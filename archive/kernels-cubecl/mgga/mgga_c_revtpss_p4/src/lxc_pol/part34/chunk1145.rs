//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1145/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1145<F: Float>(t5872: F, t84: F, t77: F, t5819: F, t603: F, t5826: F, t5816: F, t1923: F, t1928: F, t25157: F, t28127: F, t28138: F, t28151: F, t28154: F, t29513: F, t29526: F, t29529: F, t29533: F, t29538: F, t29544: F, t6958: F, t7702: F, t7706: F, t7709: F, t7716: F, t7720: F) -> (F, F, F, F, F, F) {
    let t29547 = t84 * t5872;
    let t29548 = t77 * t29547;
    let t29551 = t603 * t5819;
    let t29554 = t603 * t5826;
    let t29561 = t84 * t5816;
    let t29562 = t77 * t29561;
    let t29567 = -t29513 * t1928 / F::cast_from(6.0_f64) - t7702 * t7716 / F::cast_from(3.0_f64) - t7702 * t7720 / F::cast_from(3.0_f64) - t1923 * t29526 / F::cast_from(6.0_f64) - t1923 * t29529 / F::cast_from(3.0_f64) - t1923 * t29533 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28138 * t7706 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29538 * t1928 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28127 * t7706 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t6958 * t29544 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t29548 + t29551 * t1928 / F::cast_from(3.0_f64) + t29554 * t1928 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t7716 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t7720 - F::cast_from(5.0_f64) * t25157 * t29562 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t28151;
    (t29547, t29548, t29551, t29554, t29562, t29567)
}
