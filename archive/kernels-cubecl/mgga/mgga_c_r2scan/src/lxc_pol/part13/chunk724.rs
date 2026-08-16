//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 724/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk724<F: Float>(t1906: F, t5448: F, t652: F, t182: F, t189: F, t5686: F, t1890: F, t1893: F, t2008: F, t681: F, t1719: F, t1923: F, t1945: F, t1956: F, t2005: F, t2006: F, t201: F, t207: F, t5549: F, t5589: F, t5709: F, t5710: F, t5714: F, t5717: F, t5720: F, t689: F, t704: F, t740: F) -> (F, F, F, F) {
    let t5727 = F::cast_from(0.20620314113223568463e2_f64) * t1906 * t5448 * t652;
    let t5736 = F::cast_from(0.2137e0_f64) * t182 * t5686 * t189;
    let t5739 = F::cast_from(0.11053848960848725644e3_f64) * t1890 * t5448 * t1893;
    let t5740 = t681 * t2008;
    let t5744 = -F::cast_from(0.3903689268108626343e0_f64) * t704 * t1719 * t740 - t5709 - F::cast_from(0.14035736694323150897e2_f64) * t1945 * t5710 - F::cast_from(0.11558335953042377058e2_f64) * t5714 + F::cast_from(0.11407595979765752406e3_f64) * t5717 + F::cast_from(0.65061487801810439052e-1_f64) * t5720 - F::cast_from(0.39654301768696105266e2_f64) * t1956 * t5589 * t689 + t5727 + F::cast_from(0.20548e0_f64) * t201 * t5549 * t207 + F::cast_from(0.42514644538609193175e3_f64) * t2005 * t5589 * t2008 - t5736 - t5739 + F::cast_from(0.6207121550312808036e4_f64) * t2006 * t5740 * t1923;
    (t5727, t5736, t5739, t5744)
}
