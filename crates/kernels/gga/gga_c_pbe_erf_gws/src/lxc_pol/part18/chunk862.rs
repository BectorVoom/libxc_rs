//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 862/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk862<F: Float>(t10313: F, t220: F, t186: F, t616: F, t1019: F, t2826: F, t7048: F, t995: F, t2561: F, t5218: F, t10392: F, t562: F, t7055: F, t572: F, t7514: F, t610: F) -> (F, F, F, F, F) {
    let t10664 = -t10313;
    let t10665 = t220 * t10664;
    let t10666 = t186 * t10665;
    let t10668 = 4.0 / 15.0 * t616 * t10666;
    let t10670 = 4.0 / 15.0 * t2826 * t1019;
    let t10671 = t7048 * t995;
    let t10672 = t10671 * t2561;
    let t10674 = 16.0 / 27.0 * t5218 * t10672;
    let t10676 = t7055 * t10392 * t562;
    let t10678 = 16.0 / 45.0 * t5218 * t10676;
    let t10679 = t7514 * t572;
    let t10681 = t10679 * t10392 * t610;
    (t10668, t10670, t10674, t10678, t10681)
}
