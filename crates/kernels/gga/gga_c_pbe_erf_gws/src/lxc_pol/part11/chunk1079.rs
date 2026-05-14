//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1079/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1079<F: Float>(t18850: F, t18853: F, t18863: F, t18920: F, t18924: F, t18933: F, t18939: F, t18941: F, t19525: F, t48497: F, t48498: F, t48499: F, t49425: F, t49426: F, t18885: F, t18950: F, t18954: F, t18956: F, t18959: F, t18961: F, t18968: F, t19537: F, t48502: F, t48503: F, t48504: F, t48506: F, t48507: F, t48508: F) -> (F, F) {
    let t49427 = t18850 + t18920 + t18924 + t18853 - t49425 - t48497 - t48498 + t48499 - t18863 + t19525 - t49426 - t18933 + t18939 + t18941;
    let t49429 = -t19537 + t48502 + t48503 - t18950 + t18954 - t48504 + t48506 + t48507 + t18956 + t18885 - t48508 + t18959 - t18961 - t18968;
    (t49427, t49429)
}
