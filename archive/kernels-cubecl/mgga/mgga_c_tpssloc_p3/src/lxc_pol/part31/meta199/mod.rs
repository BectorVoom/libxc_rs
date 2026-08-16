//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta199 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk884;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk885;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk886;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk887;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk888;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk889;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk890;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta199<F: Float>(t3870: F, t5308: F, t820: F, t1367: F, t5187: F, t1341: F, t1363: F, t1831: F, t3781: F, t3783: F, t3800: F, t3803: F, t3864: F, t3867: F, t5259: F, t5289: F, t5293: F, t5303: F, t5306: F, t5257: F, t539: F, t1835: F, t225: F, t1385: F, t1842: F, t3887: F, t3787: F, t68: F, t544: F, t1824: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t5310 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk884::<F>(t3870, t5308, t820);
        let t5314 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk885::<F>(t1367, t5187, t820);
        let t5317 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk886::<F>(t1341, t1363, t1831, t3781, t3783, t3800, t3803, t3864, t3867, t5259, t5289, t5293, t5303, t5306, t5310, t5314);
        let t5318 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk887::<F>(t5257, t5317);
        let (t5319, t5321) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk888::<F>(t5318, t539, t1835, t225);
        let (t5325, t5326) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk889::<F>(t1385, t1842, t3887);
        let (t5333, t5334) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk890::<F>(t3787, t68, t544);
        let t5335 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk891::<F>(t1824, t562);
    (t5310, t5314, t5318, t5319, t5321, t5325, t5326, t5333, t5334, t5335)
}
