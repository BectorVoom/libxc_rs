//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3245/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3245<F: Float>(t10845: F, t18531: F, t18618: F, t2741: F, t18622: F, t14785: F, t18627: F, t2745: F, t2747: F, t2749: F, t2754: F, t50351: F, t5962: F, t61550: F, t61560: F, t61564: F, t61568: F, t61570: F, t836: F) -> F {
    let t61572 = t10845 * t18531;
    let t61574 = t2741 * t18618;
    let t61576 = t10845 * t18622;
    let t61578 = F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t18627 * t2754 - F::cast_from(0.80031500487063509015e-2_f64) * t61550 + F::cast_from(0.2032800112371413129e-3_f64) * t50351 - F::cast_from(0.85748036236139473944e-2_f64) * t2745 * t14785 * t5962 * t836 * t2749 - F::cast_from(0.28582678745379824648e-4_f64) * t61560 - F::cast_from(0.57165357490759649296e-4_f64) * t61564 - F::cast_from(0.57165357490759649296e-4_f64) * t61568 - F::cast_from(0.56688979511669985553e-2_f64) * t61570 + F::cast_from(0.13552000749142754193e-3_f64) * t61572 + F::cast_from(0.20007875121765877254e-2_f64) * t61574 + F::cast_from(0.13552000749142754193e-3_f64) * t61576;
    t61578
}
