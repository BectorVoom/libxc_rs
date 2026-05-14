//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 699/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk699<F: Float>(t116: F, t8545: F, t428: F, t3016: F, t385: F, t375: F, t373: F, t376: F, t383: F, t3145: F, t56: F, t136: F, t3086: F, t209: F, t371: F, t681: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8546 = t116 * t8545;
    let t8548 = 5.0 / 1296.0 * t428 * t8546;
    let t8581 = 1.0 / t3016 / t385;
    let t8582 = t375 * t8581;
    let t8611 = 1.0/pow_3_2(t373);
    let t8617 = 1.0 / t376 / t383 / 4.0;
    let t8620 = t56 * t3145;
    let t8634 = t136 * t3086;
    let t8639 = t209 * t681 * t371;
    (t8546, t8548, t8581, t8582, t8611, t8617, t8620, t8634, t8639)
}
