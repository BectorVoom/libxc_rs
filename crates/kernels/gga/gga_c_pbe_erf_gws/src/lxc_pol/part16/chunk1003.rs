//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1003/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1003<F: Float>(t858: F, t8989: F, t3065: F, t8988: F, t343: F, t8827: F, t6672: F, t2169: F, t887: F, t856: F, t3108: F, t8958: F, t8960: F, t8965: F, t8969: F, t8971: F, t8973: F, t8977: F, t8980: F, t8985: F) -> (F, F, F, F, F, F) {
    let t8990 = t858 * t8989;
    let t8991 = t3065 * t8990;
    let t8993 = t8988 * t8991 / F::new(24.0);
    let t8994 = t8827 * t343;
    let t8995 = t858 * t8994;
    let t8996 = t3065 * t8995;
    let t8998 = t6672 * t8996 / F::new(48.0);
    let t8999 = t2169 * t887;
    let t9000 = t856 * t8999;
    let t9002 = t3108 * t9000 / F::new(24.0);
    let t9003 = -t8958 + t8960 - t8965 - t8969 + t8971 + t8973 - t8977 + t8980 + t8985 + t8993 - t8998 - t9002;
    (t8991, t8993, t8996, t8998, t9002, t9003)
}
