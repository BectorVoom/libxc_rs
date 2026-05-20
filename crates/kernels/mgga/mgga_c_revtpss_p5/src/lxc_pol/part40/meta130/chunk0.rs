//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 636/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk636<F: Float>(t2944: F, t954: F, t2846: F, t2904: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2898: F, t2900: F, t2906: F, t2910: F, t2913: F, t2916: F) -> (F, F, F, F) {
    let t2945 = t2944 * t954;
    let t2950 = F::cast_from(0.68863333333333333333e0_f64) * t2846;
    let t2957 = F::cast_from(0.17365833333333333333e0_f64) * t2904;
    let t2962 = -F::new(0.17648625e1) * t2882 + F::new(0.3529725e1) * t2890 + t2950 + F::cast_from(0.34431666666666666666e0_f64) * t2848 - F::cast_from(0.34431666666666666667e0_f64) * t2855 + F::new(0.103295e1) * t2860 - F::new(0.516475e0) * t2864 + F::new(0.31558125e0) * t2898 + F::new(0.6311625e0) * t2900 + t2957 + F::cast_from(0.13892666666666666667e0_f64) * t2906 - F::cast_from(0.34731666666666666667e-1_f64) * t2910 + F::new(0.20839e0) * t2913 - F::new(0.104195e0) * t2916;
    (t2945, t2950, t2957, t2962)
}
