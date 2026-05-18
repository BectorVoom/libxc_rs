//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1182/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1182<F: Float>(t3800: F, t12625: F, t458: F, t13180: F, t493: F, t10296: F, t602: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F) -> (F, F, F, F, F, F, F) {
    let t44125 = t3800 * t3800;
    let t44126 = F::new(1.0) / t44125;
    let t44841 = F::new(1.0) / t12625 / t458;
    let t45551 = F::new(1.0) / t13180 / t493;
    let t45955 = t10296 * t602;
    let t45958 = t2240 * t2246;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    (t44126, t44841, t45551, t45955, t45958, t45963, t45970)
}
