//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1772;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1773;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1774;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta435(t5480: f64, t9398: f64, t662: f64, t1449: f64, t2: f64, t584: f64, t2349: f64, t5484: f64, t19503: f64, t103: f64, t100: f64, t12774: f64, t12795: f64, t1447: f64, t19489: f64, t19493: f64, t19499: f64, t19504: f64, t4060: f64, t4064: f64, t5469: f64, t5472: f64, t5475: f64, t657: f64, t663: f64, t92: f64, t656: f64, t12747: f64, t12750: f64, t12752: f64, t19471: f64, t19474: f64, t19477: f64, t19480: f64, t19483: f64, t64: f64, t9358: f64, t9359: f64, t109: f64, t1268: f64, t12725: f64, t1458: f64, t19450: f64, t19451: f64, t19456: f64, t19461: f64, t2314: f64, t4028: f64, t4072: f64, t5113: f64, t5493: f64, t671: f64, t7676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19514, t19518, t19522, t19525, t19526, t19529) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1772(t5480, t9398, t662, t1449, t2, t584, t2349, t5484, t19503, t103, t100, t12774, t12795, t1447, t19489, t19493, t19499, t19504, t4060, t4064, t5469, t5472, t5475, t657, t663, t92);
        let (t19530, t19533) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1773(t19529, t656, t12747, t12750, t12752, t19471, t19474, t19477, t19480, t19483, t64, t9358, t9359);
        let t19534 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1774(t109, t19533);
        let t19537 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1775(t1268, t12725, t1458, t19450, t19451, t19456, t19461, t19534, t2314, t4028, t4072, t5113, t5493, t671, t7676);
    (t19514, t19518, t19522, t19525, t19526, t19529, t19530, t19534, t19537)
}
