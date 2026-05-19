//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 998/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk998<F: Float>(t35286: F, t33953: F, t4210: F, t13364: F, t31057: F, t13299: F, t31115: F, t33938: F, t7433: F, t8779: F, t1181: F, t21955: F, t30806: F, t599: F) -> (F, F, F, F, F, F) {
    let t35287 = F::cast_from(0.42874018118069736972e-3_f64) * t35286;
    let t35288 = t33953 * t4210;
    let t35290 = t31057 * t13364 * t35288;
    let t35291 = F::cast_from(0.21437009059034868486e-3_f64) * t35290;
    let t35301 = t31115 * t13299 * t33938;
    let t35302 = F::cast_from(0.15724046144802076034e-2_f64) * t35301;
    let t35307 = t7433 * t8779;
    let t35308 = F::cast_from(0.25724410870841842184e-2_f64) * t35307;
    let t35315 = t30806 * t1181 * t599 * t21955;
    (t35287, t35288, t35291, t35302, t35308, t35315)
}
