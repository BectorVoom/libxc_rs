//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 945/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk945<F: Float>(t118: F, t1986: F, t2318: F, t495: F, t34857: F, t2868: F, t35612: F, t35617: F, t35619: F, t35622: F, t35625: F, t35629: F, t35633: F, t40214: F, t40217: F, t40222: F, t40227: F, t40232: F, t40237: F, t40242: F, t5928: F, t7538: F, t7568: F) -> F {
    let t40246 = t1986 * t118 * t2318 * t495;
    let t40247 = t34857 * t40246;
    let t40249 = -t35612 + t35617 - t35619 + t35622 + F::cast_from(0.72042316457491791906e-3_f64) * t35625 + F::cast_from(0.60975299583150056628e-3_f64) * t35629 + F::cast_from(0.60975299583150056628e-3_f64) * t35633 + F::cast_from(0.79828278012425390428e-1_f64) * t5928 * t7568 - F::cast_from(0.11974241701863808564e0_f64) * t2868 * t7538 - F::cast_from(0.72042316457491791906e-3_f64) * t40214 - F::cast_from(0.72042316457491791906e-3_f64) * t40217 - F::cast_from(0.31923449919973379548e-4_f64) * t40222 - F::cast_from(0.1064114997332445985e-4_f64) * t40227 + F::cast_from(0.1064114997332445985e-4_f64) * t40232 - F::cast_from(0.71827762319940103985e-4_f64) * t40237 - F::cast_from(0.23942587439980034662e-4_f64) * t40242 + F::cast_from(0.23942587439980034662e-4_f64) * t40247;
    t40249
}
