//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 799/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk799(t12256: f64, t3470: f64, t12207: f64, t955: f64, t13121: f64, t13124: f64, t13127: f64, t13132: f64, t13134: f64, t13138: f64, t13140: f64, t13144: f64, t13147: f64, t13152: f64, t13156: f64, t13160: f64, t13163: f64) -> f64 {
    let t13904 = t12256 * t3470;
    let t13906 = t955 * t12207;
    let t13912 = -0.10725146985555128001e1_f64 * t13904 + 0.35750489951850426669e0_f64 * t13906 + 0.14896037479937677779e-1_f64 * t13121 + t13124 - 0.46011511144704899612e1_f64 * t13127 - t13132 + 0.11502877786176224903e2_f64 * t13134 + t13138 + t13140 + t13144 - 0.14896037479937677779e-1_f64 * t13147 - t13152 + t13156 - t13160 - t13163;
    t13912
}
