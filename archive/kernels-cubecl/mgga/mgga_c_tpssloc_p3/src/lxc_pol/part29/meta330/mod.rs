//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta330<F: Float>(t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t248: F, t3516: F, t3570: F, t3515: F, t3576: F, t3604: F) -> (F, F, F, F, F, F) {
        let (t11644, t11647, t11649, t11651, t11652, t11665) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1388::<F>(t1203, t3540, t2393, t374, t486, t485, t248, t3516, t3570, t3515, t3576, t3604);
    (t11644, t11647, t11649, t11651, t11652, t11665)
}
