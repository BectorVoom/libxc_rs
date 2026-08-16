//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta431<F: Float>(t4680: F, t4684: F, t11060: F, t3040: F, t1629: F, t4673: F, t1049: F, t4649: F, t1060: F, t11066: F, t1615: F, t3166: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14574, t14577, t14578, t14581, t14586, t14587, t14590, t14591, t14595) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1851::<F>(t4680, t4684, t11060, t3040, t1629, t4673, t1049, t4649, t1060, t11066, t1615, t3166);
    (t14574, t14577, t14578, t14581, t14586, t14587, t14590, t14591, t14595)
}
