//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1146/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1146<F: Float>(t5: F, t29567: F, t117: F, t1931: F, t5883: F, t2034: F, t22475: F, t2014: F, t7898: F, t7901: F, t4248: F, t7742: F, t28172: F, t7900: F) -> (F, F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t29568 = piecewise3::<F>(t8, F::new(0.0), t29567);
    let t29569 = t29568 * t117;
    let t29573 = t1931 * t5883;
    let t29576 = t2034 * t22475;
    let t29578 = F::new(2.0) * t2014 * t29576;
    let t29580 = F::new(6.0) * t7898 * t7901;
    let t29582 = F::new(4.0) * t4248 * t7742;
    let t29583 = t28172 * t7900;
    (t29568, t29569, t29573, t29576, t29578, t29580, t29582, t29583)
}
