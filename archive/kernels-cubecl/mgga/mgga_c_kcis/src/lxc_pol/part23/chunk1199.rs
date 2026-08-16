//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1199/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1199<F: Float>(t1628: F, t27671: F, t27733: F, t26656: F, t13093: F, t2167: F, t4527: F, t7671: F, t93826: F, t1655: F, t26654: F, t28311: F) -> (F, F, F, F, F, F, F, F) {
    let t95235 = t27671 * t1628;
    let t95271 = F::cast_from(2.0_f64) * t27733;
    let t95275 = F::cast_from(4.0_f64) * t26656;
    let t97548 = t13093 * t2167;
    let t97561 = F::cast_from(2.0_f64) * t4527 * t7671;
    let t97584 = F::cast_from(2.0_f64) * t93826;
    let t97601 = t1655 * t26654;
    let t97622 = t28311 / F::cast_from(8.0_f64);
    (t95235, t95271, t95275, t97548, t97561, t97584, t97601, t97622)
}
