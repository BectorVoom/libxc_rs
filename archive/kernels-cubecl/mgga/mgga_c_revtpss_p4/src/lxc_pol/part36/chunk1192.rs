//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1192/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1192<F: Float>(t30681: F, t72: F, t1927: F, t7719: F, t8143: F, t2122: F, t29532: F, t1923: F, t2123: F, t26792: F, t28154: F, t29380: F, t29388: F, t29412: F, t29513: F, t29538: F, t29544: F, t29548: F, t29551: F, t29554: F, t29562: F, t7566: F, t7702: F, t7706: F, t7709: F, t8144: F, t8147: F) -> (F, F, F, F, F) {
    let t30682 = t30681 * t72;
    let t30683 = t30682 * t1927;
    let t30686 = t8143 * t7719;
    let t30689 = t2122 * t29532;
    let t30714 = -t29513 * t2123 / F::cast_from(6.0_f64) - t7702 * t8144 / F::cast_from(3.0_f64) - t7702 * t8147 / F::cast_from(3.0_f64) - t1923 * t30683 / F::cast_from(6.0_f64) - t1923 * t30686 / F::cast_from(3.0_f64) - t1923 * t30689 / F::cast_from(6.0_f64) - F::cast_from(5.0_f64) * t26792 * t29562 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t29380 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t29388 * t7706 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29538 * t2123 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t29412 * t7706 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7566 * t29544 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t29548 + t29551 * t2123 / F::cast_from(3.0_f64) + t29554 * t2123 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t8144 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t8147;
    (t30682, t30683, t30686, t30689, t30714)
}
