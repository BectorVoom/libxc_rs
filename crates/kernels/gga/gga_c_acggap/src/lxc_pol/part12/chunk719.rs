//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 719/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk719<F: Float>(t1167: F, t7647: F, t1103: F, t1998: F, t1108: F, t1113: F, t137: F, t922: F, t1095: F, t4352: F, t598: F, t1086: F, t2001: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7648 = t7647 * t1167;
    let t7649 = F::cast_from(0.85748036236139473944e-3_f64) * t7648;
    let t7650 = t1998 * t1103;
    let t7651 = F::cast_from(0.34299214494455789578e-2_f64) * t7650;
    let t7652 = t1998 * t1108;
    let t7653 = F::cast_from(0.17149607247227894789e-2_f64) * t7652;
    let t7654 = t1998 * t1113;
    let t7655 = F::cast_from(0.17149607247227894789e-2_f64) * t7654;
    let t7656 = t137 * t922;
    let t7658 = t4352 * t1095 * t7656;
    let t7659 = t598 * t7658;
    let t7661 = t2001 * t1086;
    (t7648, t7649, t7650, t7651, t7652, t7653, t7654, t7655, t7656, t7658, t7659, t7661)
}
