//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1108;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1109;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1110;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1111;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1112;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1113;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1114;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1115;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta233<F: Float>(t1343: F, t5287: F, t820: F, t1352: F, t5248: F, t5249: F, t120: F, t1799: F, t3805: F, t1831: F, t3866: F, t1307: F, t3870: F, t1367: F, t5187: F, t1341: F, t1363: F, t3781: F, t3783: F, t3800: F, t3803: F, t3864: F, t3867: F, t5259: F, t5257: F, t539: F, t1835: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t5289 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1108::<F>(t1343, t5287, t820);
        let t5293 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1109::<F>(t1352, t5248, t5249);
        let (t5301, t5303) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1110::<F>(t120, t1799, t1352, t3805);
        let (t5306, t5308) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1111::<F>(t1831, t3866, t1307, t1799);
        let t5310 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1112::<F>(t3870, t5308, t820);
        let t5314 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1113::<F>(t1367, t5187, t820);
        let t5317 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1114::<F>(t1341, t1363, t1831, t3781, t3783, t3800, t3803, t3864, t3867, t5259, t5289, t5293, t5303, t5306, t5310, t5314);
        let t5318 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1115::<F>(t5257, t5317);
        let (t5319, t5321) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1116::<F>(t5318, t539, t1835, t225);
    (t5289, t5293, t5301, t5303, t5308, t5310, t5314, t5318, t5319, t5321)
}
