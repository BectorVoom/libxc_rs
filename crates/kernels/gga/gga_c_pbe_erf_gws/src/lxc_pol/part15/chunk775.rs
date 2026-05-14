//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 775/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk775<F: Float>(t2609: F, t4913: F, t213: F, t331: F, t34: F, t649: F, t661: F, t1620: F, t267: F, t4872: F, t4873: F, t4876: F, t4910: F, t6971: F, t6995: F, t6998: F, t7002: F, t7007: F, t7008: F, t7009: F, t7010: F, t7013: F, t7015: F) -> (F, F, F) {
    let t7017 = 8.0 / 15.0 * t4913 * t2609;
    let t7018 = t331 * t213;
    let t7019 = t649 * t34;
    let t7020 = t7019 * t661;
    let t7021 = t7018 * t7020;
    let t7023 = 8.0 / 15.0 * t1620 * t7021;
    let t7024 = -t4872 - t6971 - t6995 * t267 / 15.0 + 2.0 / 135.0 * t6998 + 0.66490888888888888888e-1 * t4873 + t4876 + t7002 - t7007 + t7008 + t4910 + t7009 + t7010 - t7013 + t7015 - t7017 + t7023;
    (t7017, t7023, t7024)
}
