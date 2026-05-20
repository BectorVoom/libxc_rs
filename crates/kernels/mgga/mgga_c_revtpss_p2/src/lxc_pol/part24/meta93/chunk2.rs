//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 539/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk539<F: Float>(t2846: F, t307: F, t944: F, t302: F, t2904: F) -> (F, F, F, F, F, F, F) {
    let t2930 = F::cast_from(0.22831111111111111111e-1_f64) * t2846;
    let t2941 = t944 * t307;
    let t2942 = F::new(1.0) / t2941;
    let t2943 = t302 * t2942;
    let t2950 = F::cast_from(0.68863333333333333333e0_f64) * t2846;
    let t2957 = F::cast_from(0.17365833333333333333e0_f64) * t2904;
    let t2966 = t944 * t944;
    (t2930, t2941, t2942, t2943, t2950, t2957, t2966)
}
