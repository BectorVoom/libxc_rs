//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3217/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3217<F: Float>(t45: F, t39438: F, t49876: F, t11064: F, t6075: F, t37: F, t5940: F, t2612: F, t10446: F, t13312: F, t13396: F, t14401: F, t18272: F, t18277: F, t18281: F, t2251: F, t2258: F, t2375: F, t39825: F, t4377: F, t5819: F, t5825: F, t606: F, t60717: F, t60754: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t61031 = F::cast_from(0.32530743900905219526e-1_f64) * t39438;
    let t61032 = F::new(48.0) * t49876;
    let t61033 = t6075 * t11064;
    let t61037 = t37 * t5940;
    let t61039 = F::new(12.0) * t61037 * t2612;
    let t61062 = piecewise3::<F>(t151, F::new(0.0), F::new(40.0) / F::new(81.0) * t39825 * t5819 * t2251 - F::new(32.0) / F::new(27.0) * t14401 * t13396 - F::new(8.0) / F::new(27.0) * t18272 * t2258 + F::new(8.0) / F::new(9.0) * t2375 * t60717 + F::new(8.0) / F::new(9.0) * t4377 * t13312 - F::new(8.0) / F::new(27.0) * t10446 * t5825 * t2251 + F::new(8.0) / F::new(9.0) * t2375 * t18281 * t606 + F::new(4.0) / F::new(9.0) * t18277 * t2258 + F::new(4.0) / F::new(3.0) * t78 * t60754);
    (t61031, t61032, t61033, t61039, t61062)
}
