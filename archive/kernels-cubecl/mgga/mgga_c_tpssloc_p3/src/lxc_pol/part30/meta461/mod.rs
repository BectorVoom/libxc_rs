//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1740;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta461<F: Float>(t23228: F, t6554: F, t23171: F, t23168: F, t6556: F, t6547: F, t6573: F, t214: F, t852: F, t6568: F, t23030: F, t6563: F, t6567: F, t794: F, t6562: F, t1883: F, t23012: F, t213: F, t225: F, t252: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23229, t23231, t23232, t23235, t23237) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1740::<F>(t23228, t6554, t23171, t23168, t6556, t6547, t6573, t214, t852);
        let (t23249, t23252, t23253, t23254, t23262, t23270) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1741::<F>(t6547, t6568, t23030, t6563, t6567, t794, t6562, t1883, t23012, t213, t225, t252);
    (t23229, t23231, t23232, t23235, t23237, t23249, t23252, t23253, t23254, t23262, t23270)
}
