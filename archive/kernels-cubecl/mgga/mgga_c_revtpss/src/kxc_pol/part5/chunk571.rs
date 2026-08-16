//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 571/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk571<F: Float>(t290: F, t2846: F, t941: F, t945: F, t307: F, t944: F, t302: F, t2904: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2925 = t290 * t290;
    let t2926 = F::cast_from(1.0_f64) / t2925;
    let t2930 = F::cast_from(0.22831111111111111111e-1_f64) * t2846;
    let t2938 = t941 * t945;
    let t2941 = t944 * t307;
    let t2942 = F::cast_from(1.0_f64) / t2941;
    let t2943 = t302 * t2942;
    let t2950 = F::cast_from(0.68863333333333333333e0_f64) * t2846;
    let t2957 = F::cast_from(0.17365833333333333333e0_f64) * t2904;
    let t2966 = t944 * t944;
    let t2967 = F::cast_from(1.0_f64) / t2966;
    let t2968 = t302 * t2967;
    let t2969 = t310 * t310;
    (t2925, t2926, t2930, t2938, t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969)
}
