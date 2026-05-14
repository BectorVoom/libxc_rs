//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 949/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk949<F: Float>(t1140: F, t4645: F, t1137: F, t4632: F, t13273: F, t515: F, t1456: F, t3237: F, t4759: F, t997: F, t4518: F, t4574: F, t4583: F, t4587: F, t14173: F, t4741: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18336 = t1140 * t4645;
    let t18338 = t1137 * t4632;
    let t18340 = t13273 * t515;
    let t18347 = t3237 * t1456;
    let t18349 = t997 * t4759;
    let t18351 = t997 * t4518;
    let t18364 = t1137 * t4574;
    let t18366 = t1137 * t4583;
    let t18368 = t1137 * t4587;
    let t18388 = t14173 * t4741;
    (t18336, t18338, t18340, t18347, t18349, t18351, t18364, t18366, t18368, t18388)
}
