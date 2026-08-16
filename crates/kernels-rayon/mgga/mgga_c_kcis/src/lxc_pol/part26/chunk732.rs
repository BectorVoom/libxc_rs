//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 732/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk732(t303: f64, t8176: f64, t2012: f64, t553: f64, t2237: f64, t2239: f64, t7898: f64, t7906: f64, t7908: f64, t7922: f64, t8144: f64, t8148: f64, t8151: f64, t8155: f64, t8159: f64, t8166: f64, t8169: f64, t8172: f64) -> (f64, f64, f64, f64) {
    let t8177 = t303 * t8176;
    let t8179 = t553 * t2012;
    let t8180 = t303 * t8179;
    let t8182 = -0.69505208333333333333e-3_f64 * t8144 * t2239 + 0.92754700520833333333e-4_f64 * t7898 * t8148 + 0.18534722222222222222e-2_f64 * t8151 * t2239 - t7906 - 0.23168402777777777778e-3_f64 * t7908 * t8155 + 0.69505208333333333333e-3_f64 * t2237 * t8159 + 0.69505208333333333333e-3_f64 * t2237 * t8148 + t7922 + 0.16581944444444444444e-2_f64 * t8166 + 0.24872916666666666666e-2_f64 * t8169 - 0.24872916666666666666e-2_f64 * t8172 - 0.66327777777777777776e-2_f64 * t8177 + 0.16581944444444444444e-2_f64 * t8180;
    (t8177, t8179, t8180, t8182)
}
