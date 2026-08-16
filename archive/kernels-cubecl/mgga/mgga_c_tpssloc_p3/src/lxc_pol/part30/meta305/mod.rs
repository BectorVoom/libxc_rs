//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1326;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1327;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta305<F: Float>(t10470: F, t10471: F, t1013: F, t363: F, t3034: F, t6793: F, t368: F, t3131: F, t360: F, t376: F, t676: F, t1023: F, t248: F, t1020: F, t2928: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t10472 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1326::<F>(t10470, t10471);
        let (t10474, t10475, t10477, t10478) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1327::<F>(t1013, t363, t3034, t6793, t368);
        let (t10480, t10482, t10508, t10510, t10511, t10523) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1328::<F>(t10475, t10478, t10472, t3131, t360, t376, t676, t1023, t248, t1020, t2928, t320);
    (t10472, t10474, t10475, t10477, t10478, t10480, t10482, t10508, t10510, t10511, t10523)
}
