//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2717/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2717<F: Float>(t45: F, t39858: F, t14386: F, t2414: F, t39860: F, t10326: F, t10356: F, t10446: F, t11231: F, t13312: F, t14401: F, t14404: F, t1469: F, t2251: F, t2258: F, t2375: F, t39825: F, t4186: F, t4377: F, t49889: F, t606: F, t78: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t49992 = F::new(12.0) * t39858;
    let t49994 = F::new(12.0) * t14386 * t2414;
    let t49995 = F::cast_from(0.17090684152272775383e-2_f64) * t39860;
    let t50014 = piecewise3::<F>(t151, F::new(0.0), F::new(40.0) / F::new(81.0) * t39825 * t1469 * t10356 - F::new(8.0) / F::new(9.0) * t10446 * t4186 * t2251 - F::new(8.0) / F::new(9.0) * t14401 * t11231 + F::new(4.0) / F::new(3.0) * t2375 * t13312 * t606 + F::new(4.0) / F::new(3.0) * t14404 * t2258 + F::new(4.0) / F::new(9.0) * t4377 * t10326 + F::new(4.0) / F::new(3.0) * t78 * t49889);
    (t49992, t49994, t49995, t50014)
}
