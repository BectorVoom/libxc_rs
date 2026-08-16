//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1748;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta464<F: Float>(t23384: F, t6707: F, t6695: F, t6680: F, t6683: F, t6699: F, t968: F, t1920: F, t225: F, t3173: F, t368: F, t3068: F, sigma0: F, t1058: F) -> (F, F, F, F, F, F, F, F) {
        let (t23385, t23387, t23389, t23392, t23394, t23417, t23418) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1748::<F>(t23384, t6707, t6695, t6680, t6683, t6699, t968, t1920, t225, t3173, t368, t3068, sigma0);
        let t23419 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1749::<F>(t1058, t23418);
    (t23385, t23387, t23389, t23392, t23394, t23417, t23418, t23419)
}
