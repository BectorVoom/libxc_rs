//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta512<F: Float>(t1760: F, t6267: F, t3598: F, t6243: F, t11606: F, t11764: F, t20234: F, t974: F, t1743: F, t6169: F, t11487: F, t14766: F, t18494: F, t18505: F, t18512: F, t21747: F, t21751: F, t21789: F, t21792: F, t21795: F, t21802: F) -> (F, F, F, F, F, F, F) {
        let (t22004, t22007, t22008, t22011, t22012, t22015, t22032) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1972::<F>(t1760, t6267, t3598, t6243, t11606, t11764, t20234, t974, t1743, t6169, t11487, t14766, t18494, t18505, t18512, t21747, t21751, t21789, t21792, t21795, t21802);
    (t22004, t22007, t22008, t22011, t22012, t22015, t22032)
}
