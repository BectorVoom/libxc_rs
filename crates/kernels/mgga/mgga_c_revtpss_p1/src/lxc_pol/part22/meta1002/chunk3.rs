//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3412/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3412<F: Float>(t19021: F, t3014: F, t11404: F, t11461: F, t15104: F, t15238: F, t15242: F, t15274: F, t15277: F, t15280: F, t15284: F, t15406: F, t19167: F, t19263: F, t19307: F, t19311: F, t2962: F, t2968: F, t2987: F, t3012: F, t41756: F, t4652: F, t4674: F, t52809: F, t52812: F, t52820: F, t52825: F, t6158: F, t63583: F, t63586: F, t63589: F, t63592: F, t63596: F, t972: F) -> F {
    let t64072 = t19021 * t3014;
    let t64101 = -F::cast_from(0.23392894490538584828e1_f64) * t2987 * t19167 * t972 + F::cast_from(0.34631718211362927518e2_f64) * t3012 * t64072 * t972 + F::cast_from(0.69263436422725855036e2_f64) * t11461 * t19307 + F::cast_from(0.20508037716432813316e4_f64) * t41756 * t19311 - t63583 - t63586 - t63589 - t63592 - t63596 - F::new(8.0) * t52809 * t4652 - F::new(8.0) * t15104 * t15274 - F::new(4.0) * t15104 * t15277 - F::cast_from(0.38596750796862084161e3_f64) * t52812 * t15280 + F::cast_from(0.12865583598954028054e3_f64) * t52820 * t4674 + F::cast_from(0.12865583598954028054e3_f64) * t15406 * t15284 + F::cast_from(0.64327917994770140268e2_f64) * t15406 * t15238 + F::cast_from(0.4138081033541872024e4_f64) * t52825 * t15242 + F::new(12.0) * t11404 * t19263 + F::new(6.0) * t2968 * t6158 * t2962;
    t64101
}
