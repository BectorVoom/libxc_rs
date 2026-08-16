//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta463<F: Float>(t1922: F, t2966: F, t1920: F, t1049: F, t6703: F, t225: F, t6710: F, t6769: F, t134: F, t221: F, t1926: F) -> (F, F, F, F, F, F, F) {
        let (t23357, t23359, t23365, t23369, t23372, t23383, t23384) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1747::<F>(t1922, t2966, t1920, t1049, t6703, t225, t6710, t6769, t134, t221, t1926);
    (t23357, t23359, t23365, t23369, t23372, t23383, t23384)
}
