//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta618<F: Float>(t3032: F, t3508: F, t24785: F, t24826: F, t7368: F, t85660: F, t24574: F, t24781: F, t24789: F, t85639: F, t11553: F, t2121: F, t2148: F) -> (F, F, F, F, F, F) {
        let (t85972, t85984, t85986, t85988, t85996, t86000) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2060::<F>(t3032, t3508, t24785, t24826, t7368, t85660, t24574, t24781, t24789, t85639, t11553, t2121, t2148);
    (t85972, t85984, t85986, t85988, t85996, t86000)
}
