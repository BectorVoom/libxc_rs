//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk575;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk576;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk577;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk578;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk579;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk580;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta93<F: Float>(t2002: F, t544: F, t559: F, t1998: F, t562: F, t214: F, t1985: F, t63: F, t67: F, t1864: F, t5: F, t1860: F, t112: F, t109: F, t1871: F, t510: F, t1888: F, t1896: F, t1900: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2003, t2004, t2009, t2010, t2011, t2031) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk575::<F>(t2002, t544, t559, t1998, t562, t214, t1985, t63, t67);
        let t2032 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk576::<F>(t1864, t2031);
        let t2035 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk577::<F>(t5, t1860, t2032);
        let t2036 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk578::<F>(t112, t2035);
        let t2039 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk579::<F>(t109, t1871);
        let t2040 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk580::<F>(t2039, t510);
        let t2047 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk581::<F>(t1888, t1896, t1900);
    (t2003, t2004, t2009, t2010, t2011, t2031, t2032, t2035, t2036, t2039, t2040, t2047)
}
