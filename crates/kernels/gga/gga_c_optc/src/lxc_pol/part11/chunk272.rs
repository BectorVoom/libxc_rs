//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 272/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk272<F: Float>(t385: F, t375: F, t376: F, t1023: F, t373: F, t222: F, t381: F, t790: F) -> (F, F, F, F, F, F, F) {
    let t1035 = t385 * t385;
    let t1036 = F::new(1.0) / t1035;
    let t1037 = t375 * t1036;
    let t1038 = F::new(1.0) / t376;
    let t1043 = F::new(0.29896666666666666667e0) * t1023;
    let t1045 = f64::sqrt(t373);
    let t1049 = t222 * t790 * t381;
    (t1035, t1036, t1037, t1038, t1043, t1045, t1049)
}
