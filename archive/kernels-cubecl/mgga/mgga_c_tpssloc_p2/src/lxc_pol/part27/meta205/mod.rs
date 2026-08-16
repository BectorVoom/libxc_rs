//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta205 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1023;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1024;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1025;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1026;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta205<F: Float>(t247: F, t375: F, t1043: F, t2775: F, t3961: F, t2770: F, t3061: F, t1615: F, t376: F, t1022: F, t3131: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4582 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1023::<F>(t247, t375);
        let (t4583, t4584, t4585) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1024::<F>(t1043, t2775, t3961, t4582);
        let (t4588, t4589, t4590) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1025::<F>(t2770, t3061, t3961, t4582);
        let t4593 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1026::<F>(t1615, t376);
        let (t4594, t4595, t4596) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1027::<F>(t1022, t3131, t4593, t4582);
    (t4582, t4583, t4584, t4585, t4588, t4589, t4590, t4593, t4594, t4595, t4596)
}
