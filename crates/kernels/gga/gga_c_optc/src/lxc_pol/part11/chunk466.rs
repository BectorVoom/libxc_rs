//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 466/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk466<F: Float>(t2843: F, t1035: F, t385: F, t375: F, t2865: F) -> (F, F, F, F, F, F, F) {
    let t2980 = F::cast_from(0.23744444444444444444e-1_f64) * t2843;
    let t2991 = t1035 * t385;
    let t2992 = F::new(1.0) / t2991;
    let t2993 = t375 * t2992;
    let t3000 = F::cast_from(0.39862222222222222223e0_f64) * t2843;
    let t3007 = F::cast_from(0.13692777777777777778e0_f64) * t2865;
    let t3016 = t1035 * t1035;
    (t2980, t2991, t2992, t2993, t3000, t3007, t3016)
}
