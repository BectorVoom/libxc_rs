//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2227/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2227<F: Float>(t5816: F, t640: F, t77: F, t29561: F, t644: F, t4241: F, t7705: F, t1927: F, t1926: F, t101219: F, t101227: F, t101237: F, t101240: F, t101243: F, t25157: F, t28090: F, t28151: F, t28154: F, t29562: F, t7709: F, t92568: F, t92684: F, t92687: F, t92690: F) -> F {
    let t108864 = t77 * t640 * t5816;
    let t108872 = t77 * t29561 * t644;
    let t108876 = t77 * t7705 * t4241;
    let t108879 = t1927 * t5816;
    let t108880 = t1926 * t108879;
    let t108889 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t28090 - F::cast_from(5.0_f64) * t92684 * t29562 - F::cast_from(5.0_f64) * t92687 * t29562 - F::cast_from(5.0_f64) * t25157 * t108864 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t101219 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t101227 + F::cast_from(35.0_f64) * t92690 * t108872 - F::cast_from(10.0_f64) * t25157 * t108876 + F::cast_from(10.0_f64) * t92568 * t108880 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101237 * t28151 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101240 * t28151 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101243 * t28151;
    t108889
}
