//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta298<F: Float>(t761: F, t9919: F, t2531: F, t2535: F, t32: F, t717: F, t2617: F, t2629: F, t813: F, t236: F, t232: F, t2632: F) -> (F, F, F, F, F, F, F) {
        let (t9921, t9922, t9929, t9967, t9971, t9972, t9975) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1316::<F>(t761, t9919, t2531, t2535, t32, t717, t2617, t2629, t813, t236, t232, t2632);
    (t9921, t9922, t9929, t9967, t9971, t9972, t9975)
}
