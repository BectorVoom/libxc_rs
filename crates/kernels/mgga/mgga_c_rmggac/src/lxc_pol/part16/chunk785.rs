//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 785/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk785<F: Float>(t36527: F, t1347: F, t2232: F, t4793: F, t703: F, t36700: F, t36752: F, t36796: F, t36801: F, t36942: F, t36983: F, t37017: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37976 = F::new(0.2439011983326002265e-2) * t36527;
    let t38029 = t1347 * t2232;
    let t38031 = t4793 * t703;
    let t38047 = F::new(0.18292589874945016987e-2) * t36700;
    let t38060 = F::new(0.30487649791575028312e-3) * t36752;
    let t38079 = F::new(0.2439011983326002265e-2) * t36796;
    let t38080 = F::new(0.11709622077411463733e-2) * t36801;
    let t38123 = F::new(0.26021382394247697185e-3) * t36942;
    let t38140 = F::new(0.13911401682674235141e-1) * t36983;
    let t38149 = F::new(0.28691693261408173224e-3) * t37017;
    (t37976, t38029, t38031, t38047, t38060, t38079, t38080, t38123, t38140, t38149)
}
