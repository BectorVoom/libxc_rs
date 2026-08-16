//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 448/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk448(t1327: f64, t31: f64, t263: f64, t78: f64, t9: f64, t1315: f64, t1323: f64, t1326: f64, t255: f64, t261: f64, t262: f64, t331: f64, t3900: f64, t4710: f64, t4712: f64, t4720: f64, t4724: f64, t4729: f64, t4732: f64, t4737: f64, t4739: f64, t4742: f64, t4747: f64, t4750: f64, t4754: f64, t4757: f64, t4765: f64, t846: f64) -> (f64, f64, f64, f64) {
    let t4766 = t1327 * t31;
    let t4767 = t78 * t263;
    let t4768 = 1.0_f64 / t4767;
    let t4773 = 1.0_f64 / t9 / t78;
    let t4781 = -6.0_f64 * t4710 * t255 + 6.0_f64 * t4712 * t4724 - 6.0_f64 * t4729 * t255 - 0.8535056841750543333e-1_f64 * t4732 * t331 - 1.0_f64 * t4720 * t255 + 3.0_f64 * t4737 * t4739 + 0.42675284208752716665e-1_f64 * t4742 * t331 - 1.0_f64 * t4747 * t255 - 0.42675284208752716665e-1_f64 * t4750 * t331 + 0.60705996076593966083e-2_f64 * t4754 * t4757 - 0.1564760420987599611e0_f64 * t1315 * t846 - 0.31914626549668908611e-4_f64 * t4765 * t4766 * t4768 + 0.22258865228084454231e-1_f64 * t1323 * t1326 * t1327 * t4773 - 0.24340717659807105061e0_f64 * t261 * t262 * t3900;
    (t4766, t4768, t4773, t4781)
}
