//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 832/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk832<F: Float>(t6964: F, t87: F, t40: F, t4: F, t959: F, t1448: F, t2551: F, t735: F, t34: F, t92: F, t93: F, t108: F, t1403: F, t1407: F, t1413: F, t1416: F, t2538: F, t2541: F, t2544: F, t2547: F, t39: F, t532: F, t6937: F, t6952: F, t726: F, t728: F, t964: F, t965: F) -> (F, F, F, F) {
    let t6965 = t6964 * t87;
    let t6966 = t40 * t6965;
    let t6967 = t959 * t4;
    let t6968 = t6967 * t1448;
    let t6969 = F::cast_from(0.10843580882781524214e-1_f64) * t6968;
    let t6971 = F::new(4.0) / F::new(45.0) * t2551 * t735;
    let t6974 = t92 * t34;
    let t6985 = t93 * t34;
    let t6995 = (F::new(40.0) / F::new(27.0) * t964 * t1403 + F::new(80.0) / F::new(9.0) * t6974 * t6937 + F::new(20.0) / F::new(9.0) * t2538 * t1407 + F::new(8.0) / F::new(3.0) * t726 * t532 - F::new(8.0) * t2541 * t39 + F::new(40.0) / F::new(27.0) * t965 * t1413 - F::new(80.0) / F::new(9.0) * t6985 * t6952 + F::new(20.0) / F::new(9.0) * t2544 * t1416 - F::new(8.0) / F::new(3.0) * t728 * t532 + F::new(8.0) * t2547 * t39) * t108;
    (t6966, t6969, t6971, t6995)
}
