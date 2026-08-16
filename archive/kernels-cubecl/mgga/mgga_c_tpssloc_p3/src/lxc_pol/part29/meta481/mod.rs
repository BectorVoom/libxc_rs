//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta481<F: Float>(t24649: F, t7324: F, t3493: F, t475: F, t68: F, t7328: F, t2131: F, t23508: F, t7325: F, t3030: F, t3502: F, t478: F) -> (F, F, F, F, F, F, F) {
        let (t24650, t24654, t24655, t24658, t24659, t24660, t24661) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1820::<F>(t24649, t7324, t3493, t475, t68, t7328, t2131, t23508, t7325, t3030, t3502, t478);
    (t24650, t24654, t24655, t24658, t24659, t24660, t24661)
}
