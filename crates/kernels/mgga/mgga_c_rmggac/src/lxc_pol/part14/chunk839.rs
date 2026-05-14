//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 839/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk839<F: Float>(t40231: F, t7717: F, t1462: F, t1971: F, t333: F, t511: F, t8517: F, t352: F, t515: F, t118: F, t1986: F, t2318: F, t495: F, t34857: F, t2868: F, t35612: F, t35617: F, t35619: F, t35622: F, t35625: F, t35629: F, t35633: F, t40214: F, t40217: F, t40222: F, t40227: F, t5928: F, t7538: F, t7568: F) -> (F,) {
    let t40232 = t7717 * t40231;
    let t40237 = t8517 * t1971 * t511 * t1462 * t333;
    let t40242 = t8517 * t1971 * t515 * t1462 * t352;
    let t40246 = t1986 * t118 * t2318 * t495;
    let t40247 = t34857 * t40246;
    let t40249 = -t35612 + t35617 - t35619 + t35622 + 0.72042316457491791906e-3 * t35625 + 0.60975299583150056628e-3 * t35629 + 0.60975299583150056628e-3 * t35633 + 0.79828278012425390428e-1 * t5928 * t7568 - 0.11974241701863808564e0 * t2868 * t7538 - 0.72042316457491791906e-3 * t40214 - 0.72042316457491791906e-3 * t40217 - 0.31923449919973379548e-4 * t40222 - 0.1064114997332445985e-4 * t40227 + 0.1064114997332445985e-4 * t40232 - 0.71827762319940103985e-4 * t40237 - 0.23942587439980034662e-4 * t40242 + 0.23942587439980034662e-4 * t40247;
    (t40249,)
}
