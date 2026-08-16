//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1378;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1379;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1380;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta343<F: Float>(t13550: F, t4386: F, t699: F, t4339: F, t690: F, t4344: F, t1540: F, t2394: F, t4348: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13551, t13552, t13563) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1378::<F>(t13550, t4386, t699, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1379::<F>(t4344, t690);
        let (t13567, t13598) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1380::<F>(t13566, t1540, t2394);
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1381::<F>(t13563, t13566, t4348, t690);
    (t13551, t13552, t13563, t13566, t13567, t13598, t13600, t13601, t13602)
}
