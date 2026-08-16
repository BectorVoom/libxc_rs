//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 714/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk714(t1906: f64, t5448: f64, t652: f64, t182: f64, t189: f64, t5686: f64, t1890: f64, t1893: f64, t2008: f64, t681: f64, t1719: f64, t1923: f64, t1945: f64, t1956: f64, t2005: f64, t2006: f64, t201: f64, t207: f64, t5549: f64, t5589: f64, t5709: f64, t5710: f64, t5714: f64, t5717: f64, t5720: f64, t689: f64, t704: f64, t740: f64) -> (f64, f64, f64, f64) {
    let t5727 = 0.20620314113223568463e2_f64 * t1906 * t5448 * t652;
    let t5736 = 0.2137e0_f64 * t182 * t5686 * t189;
    let t5739 = 0.11053848960848725644e3_f64 * t1890 * t5448 * t1893;
    let t5740 = t681 * t2008;
    let t5744 = -0.3903689268108626343e0_f64 * t704 * t1719 * t740 - t5709 - 0.14035736694323150897e2_f64 * t1945 * t5710 - 0.11558335953042377058e2_f64 * t5714 + 0.11407595979765752406e3_f64 * t5717 + 0.65061487801810439052e-1_f64 * t5720 - 0.39654301768696105266e2_f64 * t1956 * t5589 * t689 + t5727 + 0.20548e0_f64 * t201 * t5549 * t207 + 0.42514644538609193175e3_f64 * t2005 * t5589 * t2008 - t5736 - t5739 + 0.6207121550312808036e4_f64 * t2006 * t5740 * t1923;
    (t5727, t5736, t5739, t5744)
}
