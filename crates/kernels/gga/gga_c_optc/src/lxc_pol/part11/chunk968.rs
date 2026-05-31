//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 968/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk968<F: Float>(t17744: F, t389: F, t17363: F, t17425: F, t17447: F, t17453: F, t17456: F, t17460: F, t17471: F, t17504: F, t17531: F, t17733: F) -> (F, F) {
    let t17746 = F::cast_from(0.62182e-1_f64) * t17744 * t389;
    let t17747 = -t17531 + t17456 - t17733 - t17447 + t17460 - t17746 - t17453 + t17471 - t17504 + t17363 + t17425;
    (t17746, t17747)
}
