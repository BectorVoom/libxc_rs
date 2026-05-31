//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 631/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk631<F: Float>(t33: F, t1113: F, t5557: F, t5560: F, t580: F, t162: F, t5556: F, t189: F, t512: F, t1856: F, t749: F, t177: F, t762: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t5564 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5557 * t1113 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5560 * t580);
    let t5566 = (t5556 + t5564) * t162;
    let t5567 = t5566 * t189;
    let t5568 = t512 * t5567;
    let t5569 = t1856 * t749;
    let t5570 = t512 * t5569;
    let t5571 = t1856 * t177;
    let t5572 = t5571 * t762;
    (t5566, t5568, t5570, t5572)
}
