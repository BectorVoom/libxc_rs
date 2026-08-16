//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1821;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta482<F: Float>(t1011: F, t3507: F, t3508: F, t24661: F, t1209: F, t3030: F, t478: F, t475: F, t1222: F, t7334: F, t2140: F, t3566: F, t2141: F, t3540: F, t3: F, t7324: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24663, t24664, t24667, t24668, t24669, t24670, t24675, t24677) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1821::<F>(t1011, t3507, t3508, t24661, t1209, t3030, t478, t475, t1222, t7334, t2140, t3566);
        let (t24681, t24682) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1822::<F>(t2141, t3540, t3, t7324);
    (t24663, t24664, t24667, t24668, t24669, t24670, t24675, t24677, t24681, t24682)
}
