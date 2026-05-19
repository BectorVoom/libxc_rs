//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 528/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk528<F: Float>(t5: F, t1497: F, t2242: F, t2247: F, t4171: F, t4173: F, t4178: F, t4241: F, t603: F, t644: F, t91: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t4245 = piecewise3::<F>(t8, F::new(0.0), -F::new(4.0) * t1497 * t2242 + F::new(20.0) * t2247 * t4178 + t4171 * t91 - F::new(4.0) * t4173 * t644 - F::new(4.0) * t4241 * t603);
    let t4246 = t4245 * t117;
    (t4245, t4246)
}
