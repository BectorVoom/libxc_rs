//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 523/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk523<F: Float>(t169: F, t172: F, t9127: F, t452: F, t2312: F, t3122: F, t484: F, t3130: F, t493: F, t492: F, t105: F, t3088: F, t3119: F, t3126: F, t3134: F, t3138: F, t380: F, t419: F) -> (F, F, F, F, F, F) {
    let t9129 = t9127 * t169 * t172;
    let t9130 = t452 * t9129;
    let t9147 = F::cast_from(0.23712505529730124666e-2_f64) * t2312 * t3122;
    let t9149 = F::cast_from(0.31616674039640166221e-2_f64) * t484 * t3122;
    let t9151 = F::cast_from(0.31616674039640166221e-2_f64) * t484 * t3130;
    let t9152 = t493 * t9127;
    let t9153 = t492 * t9152;
    let t9158 = -F::cast_from(0.85365019907028448797e-1_f64) * t419 * t3126 - F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3138 + F::cast_from(0.7588001769513639893e-1_f64) * t380 * t3134 - F::cast_from(0.1138200265427045984e0_f64) * t380 * t3126 + F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3119 + F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3088 + t9147 - t9149 + t9151 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t9153 + F::cast_from(0.56910013271352299198e-1_f64) * t419 * t3134;
    (t9130, t9147, t9149, t9151, t9152, t9158)
}
