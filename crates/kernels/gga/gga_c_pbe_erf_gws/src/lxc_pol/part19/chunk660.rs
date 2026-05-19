//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 660/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk660<F: Float>(t985: F, t133: F, t1511: F, t1519: F, t1583: F, t2909: F, t3648: F, t3651: F, t3654: F, t3657: F, t3661: F, t138: F, t1577: F, t2902: F, t3671: F, t514: F) -> (F, F, F) {
    let t3675 = t985 * t985;
    let t3683 = -t1511 + t3648 + t1519 + t3651 - t3654 + t1583 + F::cast_from(0.11495033333333333333e1_f64) * t2909 + F::new(0.5172765e1) * t133 * t3657 - F::new(0.1724255e1) * t133 * t3661;
    let t3685 = t138 * t3671 + F::new(2.0) * t1577 * t3675 - F::new(2.0) * t2902 * t985 - t3683 * t514;
    (t3675, t3683, t3685)
}
