//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 710/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk710<F: Float>(t5: F, t2247: F, t7565: F, t55: F, t60: F, t606: F, t6971: F, t72: F, t1927: F, t2122: F, t6977: F, t1923: F, t2123: F, t6954: F, t6960: F, t6963: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t7566 = t2247 * t7565;
    let t7571 = t55 * t60;
    let t7574 = -F::new(5.0) / F::new(6.0) * t7571 * t606 + t6971;
    let t7575 = t7574 * t72;
    let t7576 = t7575 * t1927;
    let t7579 = t2122 * t6977;
    let t7583 = piecewise3::<f64>(t8, F::new(0.0), -t6954 * t2123 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t7566 * t6960 + t6963 * t2123 / F::new(3.0) - t1923 * t7576 / F::new(6.0) - t1923 * t7579 / F::new(6.0));
    (t7566, t7571, t7574, t7575, t7576, t7579, t7583)
}
