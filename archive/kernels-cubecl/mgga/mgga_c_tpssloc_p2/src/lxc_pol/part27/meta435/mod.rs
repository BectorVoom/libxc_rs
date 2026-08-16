//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1757;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1758;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta435<F: Float>(t22792: F, t22794: F, t547: F, t6546: F, t1329: F, t3770: F, t6916: F, t22754: F, t22757: F, t22762: F, t22767: F, t22768: F, t22771: F, t22774: F, t22777: F, t22780: F, t22785: F, t22786: F, t22789: F, t2230: F, t6924: F, t213: F, t6928: F, t1998: F, t236: F, t3719: F, t6926: F, t10: F, t2229: F, t60: F, t1995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22795, t22797) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1757::<F>(t22792, t22794, t547, t6546);
        let (t22798, t22802) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1758::<F>(t1329, t22797, t3770, t6916, t22754, t22757, t22762, t22767, t22768, t22771, t22774, t22777, t22780, t22785, t22786, t22789, t22795);
        let (t22803, t22804, t22805, t22808, t22809, t22811, t22813, t22814) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1759::<F>(t2230, t6924, t213, t6928, t1998, t236, t3719, t6926, t10, t2229, t60, t1995);
    (t22795, t22797, t22798, t22802, t22803, t22804, t22805, t22808, t22809, t22811, t22813, t22814)
}
