//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 495/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk495(t196: f64, t6261: f64, t231: f64, t446: f64, t1839: f64, t500: f64, t1910: f64, t195: f64, t1023: f64, t1143: f64, t1535: f64, t4155: f64, t4163: f64, t4187: f64, t4585: f64, t5385: f64, t5388: f64, t5402: f64, t5452: f64, t5981: f64, t5985: f64, t5988: f64, t5989: f64, t5990: f64, t5992: f64, t5994: f64, t6034: f64, t6039: f64) -> f64 {
    let t6262 = t196 * t6261;
    let t6265 = t446 * t231;
    let t6268 = t500 * t1839;
    let t6275 = t195 * t1910;
    let t6280 = t5981 - t5385 + 0.31091e-1_f64 * t6262 * t500 + 0.186546e0_f64 * t6265 * t1839 + t5388 - t5985 + 0.186546e0_f64 * t4585 * t6268 + t5988 + 0.186546e0_f64 * t1143 * t6039 - t4155 - t4163 - t5989 - t5990 + 0.186546e0_f64 * t5452 * t1535 + 0.93273e-1_f64 * t6275 * t1023 + 0.373092e0_f64 * t1143 * t6034 - t5402 + t5992 + t5994 + t4187;
    t6280
}
