//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1990/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1990<F: Float>(t670: F, t7356: F, t2051: F, t2371: F, t102019: F, t13426: F, t13514: F, t1518: F, t18227: F, t2055: F, t26153: F, t26399: F, t28653: F, t28658: F, t4248: F, t4292: F, t49686: F, t7359: F, t7373: F, t75485: F, t75667: F, t95357: F) -> (F, F, F) {
    let t102714 = t7356 * t670;
    let t102719 = t2051 * t2371;
    let t102738 = F::new(4.0) * t102019 * t670 + F::new(4.0) * t102714 * t1518 + F::new(2.0) * t102719 * t1518 + F::new(4.0) * t13426 * t7373 + F::new(2.0) * t13514 * t7359 + F::new(2.0) * t1518 * t95357 + F::new(4.0) * t18227 * t7373 + F::new(2.0) * t2055 * t49686 + F::new(2.0) * t2055 * t75485 + F::new(4.0) * t2055 * t75667 + F::new(2.0) * t2371 * t28653 + F::new(2.0) * t26153 * t4248 + F::new(4.0) * t26399 * t4292 + F::new(4.0) * t28658 * t4292;
    (t102714, t102719, t102738)
}
