//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1138/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1138<F: Float>(t4583: F, t4823: F, t1042: F, t1025: F, t1028: F, t15618: F, t15712: F, t15724: F, t19770: F, t19773: F, t19778: F, t19782: F, t19786: F, t3091: F, t3124: F, t3127: F, t3224: F, t4788: F, t6278: F, t6302: F) -> (F,) {
    let t19791 = t4823 * t4583;
    let t19792 = t1042 * t19791;
    let t19797 = -0.21437009059034868486e-3 * t3224 * t6278 - 0.21437009059034868486e-3 * t1025 * t19770 - 0.21437009059034868486e-3 * t19773 * t1028 + 0.28582678745379824648e-3 * t3091 * t19778 + 0.23818898954483187207e-3 * t3091 * t19782 + 0.19055119163586549765e-3 * t19786 - 0.6351706387862183255e-4 * t15712 + t15724 + 0.28582678745379824648e-3 * t15618 * t4788 - 0.28582678745379824648e-3 * t3127 * t19792 + 0.21437009059034868486e-3 * t3124 * t6302;
    (t19797,)
}
