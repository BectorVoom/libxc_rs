//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1908/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1908(t1042: f64, t19791: f64, t1025: f64, t1028: f64, t15618: f64, t15712: f64, t15724: f64, t19770: f64, t19773: f64, t19778: f64, t19782: f64, t19786: f64, t3091: f64, t3124: f64, t3127: f64, t3224: f64, t4788: f64, t6278: f64, t6302: f64) -> (f64, f64) {
    let t19792 = t1042 * t19791;
    let t19797 = -0.21437009059034868486e-3_f64 * t3224 * t6278 - 0.21437009059034868486e-3_f64 * t1025 * t19770 - 0.21437009059034868486e-3_f64 * t19773 * t1028 + 0.28582678745379824648e-3_f64 * t3091 * t19778 + 0.23818898954483187207e-3_f64 * t3091 * t19782 + 0.19055119163586549765e-3_f64 * t19786 - 0.6351706387862183255e-4_f64 * t15712 + t15724 + 0.28582678745379824648e-3_f64 * t15618 * t4788 - 0.28582678745379824648e-3_f64 * t3127 * t19792 + 0.21437009059034868486e-3_f64 * t3124 * t6302;
    (t19792, t19797)
}
