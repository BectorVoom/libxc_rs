//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1002/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1002<F: Float>(t12037: F, t1044: F, t3560: F, t11345: F, t3579: F, t11625: F, t3465: F, t3275: F, t11475: F, t3262: F, t3781: F, t885: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12038 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t12037;
    let t12039 = t3560 * t1044;
    let t12040 = t3579 * t11345;
    let t12041 = t12040 / F::cast_from(4.0_f64);
    let t12042 = t3465 * t11625;
    let t12043 = t3275 * t12042;
    let t12044 = t12043 / F::cast_from(2.0_f64);
    let t12045 = t3465 * t11475;
    let t12046 = t3262 * t12045;
    let t12047 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12046;
    let t12048 = t3781 * t885;
    (t12038, t12039, t12040, t12041, t12042, t12043, t12044, t12045, t12046, t12047, t12048)
}
