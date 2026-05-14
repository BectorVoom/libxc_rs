//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 698/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk698<F: Float>(t7923: F, t8164: F, t1394: F, t1982: F, t2243: F, t303: F, t2002: F, t7931: F, t1928: F, t552: F, t556: F, t541: F, t2012: F, t553: F, t2237: F, t2239: F, t7898: F, t7906: F, t7908: F, t7922: F, t8144: F, t8148: F, t8151: F, t8155: F, t8159: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8165 = t7923 * t8164;
    let t8166 = t1394 * t8165;
    let t8168 = t1982 * t2243;
    let t8169 = t303 * t8168;
    let t8171 = t7931 * t2002;
    let t8172 = t303 * t8171;
    let t8175 = t552 * t1928 * t556;
    let t8176 = t541 * t8175;
    let t8177 = t303 * t8176;
    let t8179 = t553 * t2012;
    let t8180 = t303 * t8179;
    let t8182 = -0.69505208333333333333e-3 * t8144 * t2239 + 0.92754700520833333333e-4 * t7898 * t8148 + 0.18534722222222222222e-2 * t8151 * t2239 - t7906 - 0.23168402777777777778e-3 * t7908 * t8155 + 0.69505208333333333333e-3 * t2237 * t8159 + 0.69505208333333333333e-3 * t2237 * t8148 + t7922 + 0.16581944444444444444e-2 * t8166 + 0.24872916666666666666e-2 * t8169 - 0.24872916666666666666e-2 * t8172 - 0.66327777777777777776e-2 * t8177 + 0.16581944444444444444e-2 * t8180;
    (t8165, t8166, t8168, t8169, t8171, t8172, t8175, t8176, t8177, t8179, t8180, t8182)
}
