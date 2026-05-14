//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 901/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk901<F: Float>(t4305: F, t5268: F, t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8885: F, t389: F, t17363: F, t17425: F, t17447: F, t17453: F, t17456: F, t17460: F, t17471: F, t17504: F, t17531: F) -> (F, F, F, F) {
    let t17733 = 0.17544670192365612213e1 * t4305 * t5268;
    let t17744 = -t8885 - 0.23744444444444444444e-1 * t11671 + 0.11872222222222222222e-1 * t14885 - 0.35616666666666666666e-1 * t14887 + 0.17808333333333333333e-1 * t14889 - 0.19787037037037037037e-1 * t17338 + 0.71233333333333333332e-1 * t17342 - 0.35616666666666666666e-1 * t17346 - 0.10685e0 * t17350 + 0.10685e0 * t17354 - 0.17808333333333333333e-1 * t17358;
    let t17746 = 0.62182e-1 * t17744 * t389;
    let t17747 = -t17531 + t17456 - t17733 - t17447 + t17460 - t17746 - t17453 + t17471 - t17504 + t17363 + t17425;
    (t17733, t17744, t17746, t17747)
}
