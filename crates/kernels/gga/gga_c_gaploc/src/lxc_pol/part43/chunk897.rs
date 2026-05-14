//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 897/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk897<F: Float>(t40775: F, t43166: F, t43168: F, t43173: F, t43179: F, t43182: F, t43185: F, t43189: F, t43195: F, t47673: F, t47677: F, t47681: F, t47685: F, t47687: F, t47690: F, t47693: F, t47696: F, t47702: F) -> (F,) {
    let t51029 = -0.15381052460284448567e-1 * t47673 + 0.15381052460284448567e-1 * t47677 + 0.18457262952341338281e0 * t47681 - 0.92286314761706691402e-1 * t47685 + 0.64087718584518535698e-3 * t47687 + 0.64087718584518535698e-3 * t47690 - 0.34180116578409885704e-2 * t47693 + 0.51270174867614828558e-2 * t47696 - t43166 - t43168 + t43173 - t43179 + t43182 + t43185 - t43189 - 0.19226315575355560709e-2 * t40775 - t43195 - 0.85450291446024714264e-3 * t47702;
    (t51029,)
}
