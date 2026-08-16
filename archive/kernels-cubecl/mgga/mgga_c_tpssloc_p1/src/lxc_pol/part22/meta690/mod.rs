//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2268;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta690<F: Float>(t16558: F, t3450: F, t11588: F, t6138: F, t3447: F, t3451: F, t4904: F, t52036: F, t15313: F, t15338: F, t18523: F, t3448: F, t6144: F, t15402: F, t18237: F, t1887: F, t337: F, t5416: F, t51968: F, t1174: F, t135: F, t18525: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t64756, t64763, t64765, t64770, t64773, t64775) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2268::<F>(t16558, t3450, t11588, t6138, t3447, t3451, t4904, t52036, t15313, t15338, t18523, t3448);
        let (t64779, t64781, t64784, t64811, t64821, t64858) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2269::<F>(t11588, t6144, t3447, t3451, t15402, t18237, t1887, t337, t5416, t4904, t51968, t1174, t135, t18525);
    (t64756, t64763, t64765, t64770, t64773, t64775, t64779, t64781, t64784, t64811, t64821, t64858)
}
