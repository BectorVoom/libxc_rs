//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 993/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk993<F: Float>(t41512: F, t41499: F, t41502: F, t41505: F, t41508: F, t41516: F, t41519: F, t41522: F, t41525: F, t41528: F, t41531: F, t41540: F, t2380: F, t2417: F, t278: F, t808: F, t9600: F) -> (F, F, F) {
    let t43631 = 0.4939111192043895748e-1 * t41512;
    let t43639 = -0.17780800291358024693e0 * t41499 + 0.88904001456790123462e-1 * t41502 + 0.1333560021851851852e0 * t41505 - 0.1333560021851851852e0 * t41508 - t43631 + 0.16669500273148148149e-1 * t41516 + 0.2469555596021947874e-1 * t41519 - 0.22226000364197530866e-1 * t41522 - 0.29634667152263374488e-1 * t41525 + 0.69147556688614540471e-1 * t41528 + 0.22226000364197530865e-1 * t41531 + 0.17286889172153635117e0 * t41540;
    let t43651 = t2417 * t2380 * t278;
    let t43656 = t808 * t9600;
    (t43639, t43651, t43656)
}
