//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 706/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk706<F: Float>(t10070: F, t236: F, t7231: F, t1970: F, t209: F, t551: F, t605: F, t3352: F, t618: F) -> (F, F, F, F, F) {
    let t10071 = t236 * t10070;
    let t10072 = t7231 * t10071;
    let t10073 = t1970 * t10072;
    let t10074 = F::new(0.85129199786595678796e-5) * t10073;
    let t10076 = t551 * t605 * t209;
    let t10077 = t236 * t10076;
    let t10078 = t3352 * t10077;
    let t10079 = t1970 * t10078;
    let t10080 = F::new(0.25538759935978703638e-4) * t10079;
    let t10082 = t618 * t551;
    (t10072, t10074, t10078, t10080, t10082)
}
