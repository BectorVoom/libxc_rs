//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 777/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk777<F: Float>(t5: F, t72: F, t8142: F, t1927: F, t2122: F, t7719: F, t1923: F, t2123: F, t7566: F, t7702: F, t7706: F, t7709: F, t117: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t8143 = t8142 * t72;
    let t8144 = t8143 * t1927;
    let t8147 = t2122 * t7719;
    let t8151 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -t7702 * t2123 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t7706 + t7709 * t2123 / F::cast_from(3.0_f64) - t1923 * t8144 / F::cast_from(6.0_f64) - t1923 * t8147 / F::cast_from(6.0_f64));
    let t8152 = t8151 * t117;
    (t8143, t8144, t8147, t8151, t8152)
}
