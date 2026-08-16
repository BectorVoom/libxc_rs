//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 523/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk523(t169: f64, t172: f64, t9127: f64, t452: f64, t2312: f64, t3122: f64, t484: f64, t3130: f64, t493: f64, t492: f64, t105: f64, t3088: f64, t3119: f64, t3126: f64, t3134: f64, t3138: f64, t380: f64, t419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9129 = t9127 * t169 * t172;
    let t9130 = t452 * t9129;
    let t9147 = 0.23712505529730124666e-2_f64 * t2312 * t3122;
    let t9149 = 0.31616674039640166221e-2_f64 * t484 * t3122;
    let t9151 = 0.31616674039640166221e-2_f64 * t484 * t3130;
    let t9152 = t493 * t9127;
    let t9153 = t492 * t9152;
    let t9158 = -0.85365019907028448797e-1_f64 * t419 * t3126 - 0.37940008847568199465e-1_f64 * t380 * t3138 + 0.7588001769513639893e-1_f64 * t380 * t3134 - 0.1138200265427045984e0_f64 * t380 * t3126 + 0.37940008847568199465e-1_f64 * t380 * t3119 + 0.37940008847568199465e-1_f64 * t380 * t3088 + t9147 - t9149 + t9151 - 0.28455006635676149599e-1_f64 * t105 * t9153 + 0.56910013271352299198e-1_f64 * t419 * t3134;
    (t9130, t9147, t9149, t9151, t9152, t9158)
}
