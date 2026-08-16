//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 922/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk922(t43598: f64, t7584: f64, t7585: f64, t43486: f64, t7427: f64, t7573: f64, t33294: f64, t9839: f64, t10062: f64, t3040: f64, t3295: f64, t8802: f64, t9800: f64) -> (f64, f64, f64, f64, f64) {
    let t43746 = 0.43710935587469654631e2_f64 * t7584 * t7585 * t43598;
    let t43750 = 0.12423108009070322895e3_f64 * t7427 * t7573 * t43486;
    let t43752 = 0.47667319935800568892e0_f64 * t33294 * t9839;
    let t43754 = 0.35750489951850426669e0_f64 * t10062 * t3040;
    let t43756 = t9800 * t8802 * t3295;
    (t43746, t43750, t43752, t43754, t43756)
}
