//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1087/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1087<F: Float>(t38068: F, t38130: F, t38143: F, t38164: F, t38175: F, t38189: F, t1120: F, t6692: F, t37038: F, t37075: F, t1299: F, t3506: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t38622 = F::cast_from(0.19634394786159580877e0_f64) * t38068;
    let t38646 = F::cast_from(0.28914548798370980346e-4_f64) * t38130;
    let t38649 = F::cast_from(0.23159605016379617484e1_f64) * t38143;
    let t38657 = F::cast_from(0.51410067763503603055e-4_f64) * t38164;
    let t38661 = F::cast_from(0.34909953929791734801e0_f64) * t38175;
    let t38666 = F::cast_from(0.46160609703545424213e1_f64) * t38189;
    let t38783 = t1120 * t6692;
    let t38792 = F::new(308.0) / F::new(27.0) * t37038;
    let t38808 = F::new(308.0) / F::new(27.0) * t37075;
    let t38839 = t3506 * t1299;
    (t38622, t38646, t38649, t38657, t38661, t38666, t38783, t38792, t38808, t38839)
}
