//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1567/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1567<F: Float>(t11703: F, t19781: F, t11710: F, t6267: F, t3091: F, t4583: F, t4823: F, t1042: F, t1025: F, t1028: F, t15618: F, t15712: F, t15724: F, t19770: F, t19773: F, t19778: F, t3124: F, t3127: F, t3224: F, t4788: F, t6278: F, t6302: F) -> F {
    let t19782 = t11703 * t19781;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    let t19791 = t4823 * t4583;
    let t19792 = t1042 * t19791;
    let t19797 = -F::cast_from(0.21437009059034868486e-3_f64) * t3224 * t6278 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t19770 - F::cast_from(0.21437009059034868486e-3_f64) * t19773 * t1028 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t19778 + F::cast_from(0.23818898954483187207e-3_f64) * t3091 * t19782 + F::cast_from(0.19055119163586549765e-3_f64) * t19786 - F::cast_from(0.6351706387862183255e-4_f64) * t15712 + t15724 + F::cast_from(0.28582678745379824648e-3_f64) * t15618 * t4788 - F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t19792 + F::cast_from(0.21437009059034868486e-3_f64) * t3124 * t6302;
    t19797
}
