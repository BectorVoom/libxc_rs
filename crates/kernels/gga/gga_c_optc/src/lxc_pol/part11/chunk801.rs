//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 801/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk801<F: Float>(t16409: F, t16442: F, t1299: F, t4675: F, t3462: F, t4723: F, t116: F, t16221: F, t6944: F, t16329: F, t696: F, t16325: F, t16323: F, t5: F, t6879: F, t675: F) -> (F, F, F, F, F, F, F, F) {
    let t16443 = t16409 + t16442;
    let t16456 = t4675 * t1299;
    let t16460 = t3462 * t4723;
    let t16464 = t6944 * t116 * t16221;
    let t16471 = t696 * t16329;
    let t16474 = t696 * t16325;
    let t16477 = t5 * t16323;
    let t16478 = t16477 * t6879;
    let t16479 = t675 * t16478;
    (t16443, t16456, t16460, t16464, t16471, t16474, t16477, t16479)
}
