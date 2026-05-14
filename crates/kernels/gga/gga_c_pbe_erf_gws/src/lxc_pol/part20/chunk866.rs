//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 866/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk866<F: Float>(t2612: F, t2667: F, t2674: F, t2680: F, t3403: F, t7011: F, t4913: F, t7580: F, t3493: F, t663: F, t1022: F, t209: F, t184: F, t2737: F, t1031: F, t617: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10730 = 8.0 / 45.0 * t2612 * t2667;
    let t10732 = 16.0 / 45.0 * t2612 * t2674;
    let t10734 = 8.0 / 27.0 * t2612 * t2680;
    let t10736 = 16.0 / 45.0 * t7011 * t3403;
    let t10738 = 16.0 / 45.0 * t4913 * t3403;
    let t10739 = 16.0 / 405.0 * t7580;
    let t10741 = 4.0 / 15.0 * t3493 * t663;
    let t10742 = t1022 * t209;
    let t10743 = t10742 * t184;
    let t10745 = 8.0 / 15.0 * t10743 * t2737;
    let t10746 = t617 * t1031;
    (t10730, t10732, t10734, t10736, t10738, t10739, t10741, t10745, t10746)
}
