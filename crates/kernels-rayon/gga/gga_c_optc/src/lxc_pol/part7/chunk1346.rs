//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1346/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1346(t26833: f64, t26846: f64, t389: f64, t1076: f64, t1086: f64, t1094: f64, t26164: f64, t26217: f64, t26556: f64, t26578: f64, t26722: f64, t26732: f64, t26735: f64, t26738: f64, t26745: f64, t26749: f64, t26754: f64, t26757: f64, t26760: f64, t26792: f64, t26805: f64, t26818: f64, t2937: f64, t2969: f64, t2976: f64, t2977: f64, t3032: f64, t3054: f64, t3061: f64, t3062: f64, t402: f64, t8700: f64, t8766: f64, t8769: f64, t8772: f64, t8773: f64, t8776: f64, t8788: f64, t8848: f64, t8854: f64) -> (f64, f64) {
    let t26849 = 0.62182e-1_f64 * (t26833 + t26846) * t389;
    let t26850 = -0.4155781415850207192e3_f64 * t26722 * t8766 + 0.6233672123775310788e3_f64 * t8772 * t26164 * t3061 + 0.35089340384731224426e1_f64 * t8854 * t3054 + 0.23392893589820816284e1_f64 * t3032 * t8769 + 0.1038945353962551798e3_f64 * t26732 * t3062 + 0.41015588084031179722e4_f64 * t26735 * t8773 - 0.12304676425209353917e5_f64 * t26738 * t26164 * t8700 + 0.58482233974552040708e0_f64 * t1086 * t26556 * t1094 + 0.91080982599109921211e5_f64 * t26745 * t26164 * t26217 + 4.0_f64 * t26749 * t1076 + 6.0_f64 * t8776 * t2969 + 0.19298809906722418784e3_f64 * t26754 * t2977 - 12.0_f64 * t26757 * t2937 - 0.77195239626889675138e3_f64 * t26760 * t8788 + 0.11579285944033451271e4_f64 * t8848 * t26578 * t2976 - 0.19751789702565206229e-1_f64 * t26792 - 0.3109e-1_f64 * (t26805 + t26818) * t402 + t26849;
    (t26849, t26850)
}
