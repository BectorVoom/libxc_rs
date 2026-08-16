//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1887;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1888;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1889;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta498<F: Float>(t1512: F, t23041: F, t4166: F, t6613: F, t831: F, t23053: F, t4236: F, t6614: F, t1878: F, t23033: F, t221: F, t4255: F, t23125: F, t23134: F, t23141: F, t23144: F, t25140: F, t25142: F, t23042: F, t23063: F, t23070: F, t23084: F, t25065: F, t25069: F, t25071: F, t25073: F, t25077: F, t25080: F, t25103: F, t25107: F, t25109: F, t25113: F, t25117: F, t25121: F, t25124: F, t25126: F, t25128: F, t25133: F, t25136: F, t218: F, t253: F, t254: F, t10109: F, t1911: F, t4272: F, t25036: F, t25042: F, t25047: F, t25049: F, t25051: F, t25056: F, t25061: F, t259: F, t2597: F, t4147: F, t4301: F, t6627: F, t6632: F, t6663: F, t7538: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25144, t25146, t25147, t25149, t25151, t25154, t25155) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1887::<F>(t1512, t23041, t4166, t6613, t831, t23053, t4236, t6614, t1878, t23033, t221, t4255);
        let t25158 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1888::<F>(t25154, t25155, t23125, t23134, t23141, t23144, t25140, t25142, t25144, t25147, t25149, t25151);
        let t25160 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1889::<F>(t23042, t23063, t23070, t23084, t25065, t25069, t25071, t25073, t25077, t25080, t25103, t25107, t25109, t25113, t25117, t25121, t25124, t25126, t25128, t25133, t25136, t25158);
        let (t25161, t25168, t25169, t25170, t25173) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1890::<F>(t218, t25160, t253, t254, t10109, t1911, t4272, t25036, t25042, t25047, t25049, t25051, t25056, t25061, t259, t2597, t4147, t4301, t6627, t6632, t6663, t7538);
    (t25146, t25154, t25155, t25160, t25161, t25168, t25169, t25170, t25173)
}
