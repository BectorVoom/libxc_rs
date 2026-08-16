//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1187/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1187(t40225: f64, t38674: f64, t544: f64, t9287: f64, t2365: f64, t38272: f64, t7025: f64, t41938: f64, t41941: f64, t41942: f64, t41945: f64, t41948: f64, t41950: f64, t41952: f64, t41954: f64, t41958: f64) -> f64 {
    let t47963 = 0.15337170381568299871e1_f64 * t40225;
    let t47964 = t544 * t38674;
    let t47965 = t47964 * t9287;
    let t47968 = t7025 * t2365 * t38272;
    let t47972 = 0.47667319935800568892e0_f64 * t41938 + t41941 + 0.35750489951850426669e0_f64 * t41942 + t41945 - t47963 + 0.14896037479937677779e-1_f64 * t47965 + 0.14896037479937677779e-1_f64 * t47968 - t41948 - t41950 - t41952 + 0.25561950635947166451e0_f64 * t41954 - 0.44688112439813033337e-1_f64 * t41958;
    t47972
}
