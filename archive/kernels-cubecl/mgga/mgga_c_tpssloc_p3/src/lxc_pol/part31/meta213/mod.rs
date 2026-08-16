//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta213<F: Float>(t2792: F, t5695: F, t1547: F, t2798: F, t2802: F, t4335: F, t5679: F, t5683: F, t5687: F, t894: F, t2815: F, t901: F) -> (F, F, F, F, F, F, F) {
        let (t5697, t5698, t5699, t5705, t5706, t5712, t5714) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk943::<F>(t2792, t5695, t1547, t2798, t2802, t4335, t5679, t5683, t5687, t894, t2815, t901);
    (t5697, t5698, t5699, t5705, t5706, t5712, t5714)
}
