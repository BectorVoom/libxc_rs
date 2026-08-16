//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1772;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1773;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1774;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta435<F: Float>(t5480: F, t9398: F, t662: F, t1449: F, t2: F, t584: F, t2349: F, t5484: F, t19503: F, t103: F, t100: F, t12774: F, t12795: F, t1447: F, t19489: F, t19493: F, t19499: F, t19504: F, t4060: F, t4064: F, t5469: F, t5472: F, t5475: F, t657: F, t663: F, t92: F, t656: F, t12747: F, t12750: F, t12752: F, t19471: F, t19474: F, t19477: F, t19480: F, t19483: F, t64: F, t9358: F, t9359: F, t109: F, t1268: F, t12725: F, t1458: F, t19450: F, t19451: F, t19456: F, t19461: F, t2314: F, t4028: F, t4072: F, t5113: F, t5493: F, t671: F, t7676: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19514, t19518, t19522, t19525, t19526, t19529) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1772::<F>(t5480, t9398, t662, t1449, t2, t584, t2349, t5484, t19503, t103, t100, t12774, t12795, t1447, t19489, t19493, t19499, t19504, t4060, t4064, t5469, t5472, t5475, t657, t663, t92);
        let (t19530, t19533) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1773::<F>(t19529, t656, t12747, t12750, t12752, t19471, t19474, t19477, t19480, t19483, t64, t9358, t9359);
        let t19534 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1774::<F>(t109, t19533);
        let t19537 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1775::<F>(t1268, t12725, t1458, t19450, t19451, t19456, t19461, t19534, t2314, t4028, t4072, t5113, t5493, t671, t7676);
    (t19514, t19518, t19522, t19525, t19526, t19529, t19530, t19534, t19537)
}
