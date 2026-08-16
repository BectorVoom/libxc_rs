//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 884/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk884(t1661: f64, t2010: f64, t8342: f64, t2415: f64, t5757: f64, t5061: f64, t38552: f64, t38554: f64, t38556: f64, t44808: f64, t44812: f64, t44816: f64, t44818: f64, t44821: f64, t44825: f64, t44828: f64, t44831: f64, t44835: f64, t44838: f64, t44841: f64) -> f64 {
    let t44844 = t2010 * t8342 * t1661;
    let t44847 = t2010 * t2415 * t5757;
    let t44850 = t2010 * t2415 * t5061;
    let t44853 = 0.25538759935978703638e-4_f64 * t44808 - 0.1064114997332445985e-4_f64 * t44812 - 0.31923449919973379548e-4_f64 * t44816 - 0.54549323308490683456e-1_f64 * t44818 - 0.72042316457491791906e-3_f64 * t44821 + 0.60975299583150056628e-3_f64 * t38552 - 0.72042316457491791906e-3_f64 * t44825 - 0.72042316457491791906e-3_f64 * t44828 + 0.60975299583150056628e-3_f64 * t38554 + 0.19211284388664477842e-2_f64 * t44831 - 0.36021158228745895953e-3_f64 * t44835 - 0.36021158228745895953e-3_f64 * t44838 - 0.36021158228745895953e-3_f64 * t44841 - 0.72042316457491791906e-3_f64 * t44844 - 0.72042316457491791906e-3_f64 * t44847 - 0.72042316457491791906e-3_f64 * t44850 - 0.70441376091769752087e-2_f64 * t38556;
    t44853
}
