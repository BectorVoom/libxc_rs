//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3722/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3722<F: Float>(t480: F, t69637: F, t20842: F, t3667: F, t17303: F, t5323: F, t12784: F, t17401: F, t17484: F, t17515: F, t17534: F, t17654: F, t17662: F, t17729: F, t17744: F, t20766: F, t21161: F, t3626: F, t3674: F, t5051: F, t56981: F, t57331: F, t57333: F, t57336: F, t57660: F, t57663: F, t57710: F) -> F {
    let t70578 = t69637 * t480;
    let t70581 = t3667 * t20842;
    let t70583 = t5323 * t17303;
    let t70593 = -F::cast_from(0.57165357490759649296e-3_f64) * t12784 * t21161 + F::cast_from(0.1270341277572436651e-3_f64) * t57331 - F::cast_from(0.3811023832717309953e-3_f64) * t57333 - F::cast_from(0.57165357490759649296e-3_f64) * t57336 + F::cast_from(0.11433071498151929859e-2_f64) * t17729 * t3626 * t5051 * t17534 - F::cast_from(0.11433071498151929859e-2_f64) * t17654 * t56981 * t20766 + F::cast_from(0.42874018118069736972e-3_f64) * t70578 * t3674 - F::cast_from(0.28582678745379824648e-3_f64) * t70581 - F::cast_from(0.5081365110289746604e-3_f64) * t70583 - F::cast_from(0.22866142996303859718e-2_f64) * t57710 * t17484 - F::cast_from(0.42874018118069736972e-3_f64) * t17401 * t17744 - F::cast_from(0.30488190661738479624e-2_f64) * t57660 * t17662 + F::cast_from(0.57165357490759649296e-3_f64) * t57663 * t17515;
    t70593
}
