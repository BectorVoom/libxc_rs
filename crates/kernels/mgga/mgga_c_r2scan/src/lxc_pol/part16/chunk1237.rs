//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1237/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1237<F: Float>(t11496: F, t2850: F, t3262: F, t3263: F, t1108: F, t2881: F, t3685: F, t42976: F, t43716: F, t43720: F, t43724: F, t43728: F, t43732: F, t43735: F, t43739: F, t43742: F, t43747: F, t43750: F, t43752: F, t43754: F, t43756: F, t9782: F) -> (F, F) {
    let t43757 = t11496 * t2850;
    let t43760 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t3263 * t43757;
    let t43761 = t1108 * t9782 + F::cast_from(2.0_f64) * t2881 * t3685 + t42976 + t43716 - t43720 + t43724 + t43728 - t43732 - t43735 - t43739 - t43742 - t43747 - t43750 - t43752 + t43754 + t43756 - t43760;
    (t43760, t43761)
}
