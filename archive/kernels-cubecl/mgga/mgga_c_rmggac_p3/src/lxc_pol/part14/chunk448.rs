//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 448/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk448<F: Float>(t1327: F, t31: F, t263: F, t78: F, t9: F, t1315: F, t1323: F, t1326: F, t255: F, t261: F, t262: F, t331: F, t3900: F, t4710: F, t4712: F, t4720: F, t4724: F, t4729: F, t4732: F, t4737: F, t4739: F, t4742: F, t4747: F, t4750: F, t4754: F, t4757: F, t4765: F, t846: F) -> (F, F, F, F) {
    let t4766 = t1327 * t31;
    let t4767 = t78 * t263;
    let t4768 = F::cast_from(1.0_f64) / t4767;
    let t4773 = F::cast_from(1.0_f64) / t9 / t78;
    let t4781 = -F::cast_from(6.0_f64) * t4710 * t255 + F::cast_from(6.0_f64) * t4712 * t4724 - F::cast_from(6.0_f64) * t4729 * t255 - F::cast_from(0.8535056841750543333e-1_f64) * t4732 * t331 - F::cast_from(1.0_f64) * t4720 * t255 + F::cast_from(3.0_f64) * t4737 * t4739 + F::cast_from(0.42675284208752716665e-1_f64) * t4742 * t331 - F::cast_from(1.0_f64) * t4747 * t255 - F::cast_from(0.42675284208752716665e-1_f64) * t4750 * t331 + F::cast_from(0.60705996076593966083e-2_f64) * t4754 * t4757 - F::cast_from(0.1564760420987599611e0_f64) * t1315 * t846 - F::cast_from(0.31914626549668908611e-4_f64) * t4765 * t4766 * t4768 + F::cast_from(0.22258865228084454231e-1_f64) * t1323 * t1326 * t1327 * t4773 - F::cast_from(0.24340717659807105061e0_f64) * t261 * t262 * t3900;
    (t4766, t4768, t4773, t4781)
}
