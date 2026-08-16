//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1744;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1745;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta441(t22792: f64, t22794: f64, t547: f64, t6546: f64, t1329: f64, t3770: f64, t6916: f64, t22754: f64, t22757: f64, t22762: f64, t22767: f64, t22768: f64, t22771: f64, t22774: f64, t22777: f64, t22780: f64, t22785: f64, t22786: f64, t22789: f64, t2230: f64, t6924: f64, t213: f64, t6928: f64, t1998: f64, t236: f64, t3719: f64, t6926: f64, t10: f64, t2229: f64, t60: f64, t1995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22795, t22797) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1744(t22792, t22794, t547, t6546);
        let (t22798, t22802) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1745(t1329, t22797, t3770, t6916, t22754, t22757, t22762, t22767, t22768, t22771, t22774, t22777, t22780, t22785, t22786, t22789, t22795);
        let (t22803, t22804, t22805, t22808, t22809, t22811, t22813, t22814) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1746(t2230, t6924, t213, t6928, t1998, t236, t3719, t6926, t10, t2229, t60, t1995);
    (t22795, t22797, t22798, t22802, t22803, t22804, t22805, t22808, t22809, t22811, t22813, t22814)
}
