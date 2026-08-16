//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2224/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2224<F: Float>(t21663: F, t607: F, t13272: F, t28126: F, t2247: F, t29524: F, t38: F, t5868: F, t644: F, t77: F, t101320: F, t1928: F, t28127: F, t28133: F, t28138: F, t28141: F, t29526: F, t29529: F, t29533: F, t6958: F, t6960: F, t6963: F, t7706: F, t7716: F, t7720: F) -> F {
    let t108769 = t21663 * t607;
    let t108772 = t13272 * t28126;
    let t108782 = t2247 * t38 * t29524;
    let t108792 = t77 * t5868 * t644;
    let t108799 = t108769 * t1928 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t108772 * t6960 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t7716 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28138 * t28133 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t7720 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t108782 * t6960 + t6963 * t29526 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28127 * t28133 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6963 * t29529 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t108792 + t6963 * t29533 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t101320 * t7706;
    t108799
}
