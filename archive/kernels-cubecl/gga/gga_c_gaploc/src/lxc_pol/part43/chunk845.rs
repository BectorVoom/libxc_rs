//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 845/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk845<F: Float>(t2487: F, t41965: F, t6711: F, t40228: F, t40239: F, t40243: F, t40249: F, t40252: F, t40258: F, t40261: F, t40277: F, t40280: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41968 = F::cast_from(0.43710935587469654631e2_f64) * t2487 * t6711 * t41965;
    let t41972 = F::cast_from(0.29792074959875355558e-1_f64) * t40228;
    let t41975 = F::cast_from(0.29792074959875355558e-1_f64) * t40239;
    let t41976 = F::cast_from(0.29792074959875355558e-1_f64) * t40243;
    let t41978 = F::cast_from(0.17041300423964777634e0_f64) * t40249;
    let t41979 = F::cast_from(0.29792074959875355558e-1_f64) * t40252;
    let t41980 = F::cast_from(0.20854452471912748891e0_f64) * t40258;
    let t41981 = F::cast_from(0.12780975317973583225e0_f64) * t40261;
    let t41982 = F::cast_from(0.17875244975925213335e0_f64) * t40277;
    let t41983 = F::cast_from(0.11916829983950142223e0_f64) * t40280;
    (t41968, t41972, t41975, t41976, t41978, t41979, t41980, t41981, t41982, t41983)
}
