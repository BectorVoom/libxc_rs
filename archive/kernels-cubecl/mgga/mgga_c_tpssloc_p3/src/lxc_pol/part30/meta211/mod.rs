//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk992;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk993;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk994;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk995;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk996;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk997;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk998;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta211<F: Float>(t225: F, t5600: F, t2671: F, t5527: F, t5544: F, t824: F, t1504: F, t1506: F, t228: F, t230: F, t232: F, t819: F, t820: F, t5584: F, t2701: F, t847: F, t1512: F, t1516: F, t249: F, t2571: F, t2602: F, t2630: F, t2643: F, t2695: F, t4152: F, t4167: F, t4170: F, t4172: F, t4187: F, t4253: F, t5568: F, t5572: F, t5576: F, t5587: F, t5593: F, t787: F, t817: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5601, t5605, t5608, t5611) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk992::<F>(t225, t5600, t2671, t5527, t5544, t824, t1504, t1506, t228, t230);
        let t5612 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk993::<F>(t232, t5611);
        let t5614 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk994::<F>(t5612, t819, t820);
        let t5617 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk995::<F>(t232, t5584);
        let t5619 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk996::<F>(t5617, t819, t820);
        let t5624 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk997::<F>(t2701, t5527, t820);
        let t5628 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk998::<F>(t5544, t820, t847);
        let t5631 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk999::<F>(t1512, t1516, t249, t2571, t2602, t2630, t2643, t2695, t4152, t4167, t4170, t4172, t4187, t4253, t5568, t5572, t5576, t5587, t5593, t5614, t5619, t5624, t5628, t787, t817, t843);
    (t5601, t5605, t5608, t5611, t5612, t5614, t5617, t5619, t5624, t5628, t5631)
}
