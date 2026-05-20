//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3676/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3676<F: Float>(t20382: F, t3523: F, t12555: F, t6534: F, t1187: F, t12481: F, t12491: F, t12553: F, t16974: F, t16979: F, t16982: F, t16985: F, t16989: F, t16992: F, t16998: F, t17032: F, t17097: F, t17150: F, t17154: F, t20659: F, t20662: F, t20671: F, t20672: F, t20675: F, t3497: F, t3515: F, t3521: F, t5163: F, t5184: F, t5185: F, t58242: F, t58247: F, t58262: F, t58307: F, t6519: F, t6538: F) -> F {
    let t69504 = t20382 * t3523;
    let t69511 = t6534 * t12555;
    let t69548 = F::cast_from(0.34631718211362927518e2_f64) * t12481 * t20672 + F::cast_from(0.34631718211362927518e2_f64) * t3521 * t69504 * t1187 + F::cast_from(0.17315859105681463759e2_f64) * t3521 * t20671 * t3515 + F::cast_from(0.10254018858216406658e4_f64) * t12553 * t69511 * t3497 + F::cast_from(0.69263436422725855036e2_f64) * t12481 * t20675 + F::cast_from(0.34631718211362927518e2_f64) * t3521 * t5184 * t17150 + F::new(12.0) * t17032 * t16974 - F::cast_from(0.46785788981077169656e1_f64) * t58307 * t5163 - F::cast_from(0.46785788981077169656e1_f64) * t17154 * t16979 - F::cast_from(0.23392894490538584828e1_f64) * t17154 * t16982 - F::cast_from(0.2077903092681775651e3_f64) * t58262 * t16985 + F::cast_from(0.69263436422725855034e2_f64) * t58242 * t5185 + F::cast_from(0.69263436422725855034e2_f64) * t17097 * t16989 + F::cast_from(0.34631718211362927517e2_f64) * t17097 * t16992 + F::cast_from(0.20508037716432813315e4_f64) * t58247 * t16998 + F::cast_from(0.70178683471615754484e1_f64) * t12481 * t20659 + F::cast_from(0.35089341735807877242e1_f64) * t3521 * t6519 * t3515 + F::cast_from(0.6233709278045326953e3_f64) * t12553 * t6538 * t3497 - F::cast_from(0.46785788981077169656e1_f64) * t12491 * t20662;
    t69548
}
