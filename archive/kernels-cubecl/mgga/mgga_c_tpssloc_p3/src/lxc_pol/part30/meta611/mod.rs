//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2006;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta611<F: Float>(t23511: F, t6733: F, t1049: F, t6743: F, t883: F, t221: F, t697: F, t1926: F, t6790: F, t6787: F, t23631: F, t974: F, t976: F, t984: F, t1009: F, t343: F, t25490: F, t210: F, t23632: F, t23668: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t82620, t82625, t82632) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2006::<F>(t23511, t6733, t1049, t6743, t883, t221, t697, t1926);
        let (t82633, t82635, t82653, t82654, t82655, t82668) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2007::<F>(t6790, t82632, t6787, t23631, t974, t976, t984, t1009, t343, t25490, t210, t23632, t23668);
    (t82620, t82625, t82632, t82633, t82635, t82653, t82654, t82655, t82668)
}
