//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1839;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1840;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1841;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1842;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta474<F: Float>(t23519: F, t23520: F, t1940: F, t3046: F, t354: F, t1046: F, t1935: F, t23489: F, t23495: F, t23500: F, t23504: F, t23510: F, t23515: F, t3057: F, t3064: F, t6723: F, t6730: F, t6735: F, t6742: F, t6747: F, t6765: F, t3053: F, t3127: F, t3037: F, t3033: F, sigma0: F, t6753: F, t1004: F, t6764: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23521, t23528, t23529) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1839::<F>(t23519, t23520, t1940, t3046, t354);
        let t23532 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1840::<F>(t1046, t1935, t23489, t23495, t23500, t23504, t23510, t23515, t23521, t23529, t3057, t3064, t6723, t6730, t6735, t6742, t6747, t6765);
        let (t23533, t23535, t23536, t23537) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1841::<F>(t3053, t6765, t3127, t3037, t3033, sigma0);
        let (t23540, t23541) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1842::<F>(t3037, t6753, t3033);
        let t23544 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1843::<F>(t1004, t6764);
    (t23521, t23528, t23529, t23532, t23533, t23535, t23536, t23537, t23540, t23541, t23544)
}
