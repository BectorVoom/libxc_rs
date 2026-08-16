//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 945/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk945(t118: f64, t1986: f64, t2318: f64, t495: f64, t34857: f64, t2868: f64, t35612: f64, t35617: f64, t35619: f64, t35622: f64, t35625: f64, t35629: f64, t35633: f64, t40214: f64, t40217: f64, t40222: f64, t40227: f64, t40232: f64, t40237: f64, t40242: f64, t5928: f64, t7538: f64, t7568: f64) -> f64 {
    let t40246 = t1986 * t118 * t2318 * t495;
    let t40247 = t34857 * t40246;
    let t40249 = -t35612 + t35617 - t35619 + t35622 + 0.72042316457491791906e-3_f64 * t35625 + 0.60975299583150056628e-3_f64 * t35629 + 0.60975299583150056628e-3_f64 * t35633 + 0.79828278012425390428e-1_f64 * t5928 * t7568 - 0.11974241701863808564e0_f64 * t2868 * t7538 - 0.72042316457491791906e-3_f64 * t40214 - 0.72042316457491791906e-3_f64 * t40217 - 0.31923449919973379548e-4_f64 * t40222 - 0.1064114997332445985e-4_f64 * t40227 + 0.1064114997332445985e-4_f64 * t40232 - 0.71827762319940103985e-4_f64 * t40237 - 0.23942587439980034662e-4_f64 * t40242 + 0.23942587439980034662e-4_f64 * t40247;
    t40249
}
