//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1291;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1292;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta287<F: Float>(t180: F, t2511: F, t9489: F, t9490: F, t761: F, t116: F, t229: F, t212: F, t776: F, t2586: F, t597: F, t60: F, t59: F, t2386: F) -> (F, F, F, F, F, F, F, F) {
        let (t9493, t9494, t9496, t9523, t9526, t9533) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1291::<F>(t180, t2511, t9489, t9490, t761, t116, t229, t212, t776, t2586, t597, t60);
        let (t9534, t9537) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1292::<F>(t59, t9533, t212, t2386);
    (t9493, t9494, t9496, t9523, t9526, t9533, t9534, t9537)
}
