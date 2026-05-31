//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1346/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1346<F: Float>(t26833: F, t26846: F, t389: F, t1076: F, t1086: F, t1094: F, t26164: F, t26217: F, t26556: F, t26578: F, t26722: F, t26732: F, t26735: F, t26738: F, t26745: F, t26749: F, t26754: F, t26757: F, t26760: F, t26792: F, t26805: F, t26818: F, t2937: F, t2969: F, t2976: F, t2977: F, t3032: F, t3054: F, t3061: F, t3062: F, t402: F, t8700: F, t8766: F, t8769: F, t8772: F, t8773: F, t8776: F, t8788: F, t8848: F, t8854: F) -> (F, F) {
    let t26849 = F::cast_from(0.62182e-1_f64) * (t26833 + t26846) * t389;
    let t26850 = -F::cast_from(0.4155781415850207192e3_f64) * t26722 * t8766 + F::cast_from(0.6233672123775310788e3_f64) * t8772 * t26164 * t3061 + F::cast_from(0.35089340384731224426e1_f64) * t8854 * t3054 + F::cast_from(0.23392893589820816284e1_f64) * t3032 * t8769 + F::cast_from(0.1038945353962551798e3_f64) * t26732 * t3062 + F::cast_from(0.41015588084031179722e4_f64) * t26735 * t8773 - F::cast_from(0.12304676425209353917e5_f64) * t26738 * t26164 * t8700 + F::cast_from(0.58482233974552040708e0_f64) * t1086 * t26556 * t1094 + F::cast_from(0.91080982599109921211e5_f64) * t26745 * t26164 * t26217 + F::cast_from(4.0_f64) * t26749 * t1076 + F::cast_from(6.0_f64) * t8776 * t2969 + F::cast_from(0.19298809906722418784e3_f64) * t26754 * t2977 - F::cast_from(12.0_f64) * t26757 * t2937 - F::cast_from(0.77195239626889675138e3_f64) * t26760 * t8788 + F::cast_from(0.11579285944033451271e4_f64) * t8848 * t26578 * t2976 - F::cast_from(0.19751789702565206229e-1_f64) * t26792 - F::cast_from(0.3109e-1_f64) * (t26805 + t26818) * t402 + t26849;
    (t26849, t26850)
}
