//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2220;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2221;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2222;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2223;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta710(t23204: f64, t28298: f64, t81640: f64, t225: f64, t28442: f64, t22986: f64, t23270: f64, t25191: f64, t4300: f64, t25192: f64, t86873: f64, t5544: f64, t857: f64, t865: f64, t1528: f64, t2597: f64, t28311: f64, t866: f64, t86951: f64, t86968: f64, t86988: f64, t92432: f64, t98234: f64, t1527: f64, t86849: f64, t4272: f64, t86969: f64, t1520: f64, t254: f64, t25038: f64, t25039: f64, t4119: f64, t1880: f64, t7488: f64, t87782: f64, t10110: f64, t17056: f64, t25168: f64, t25169: f64, t25233: f64, t25330: f64, t259: f64, t2713: f64, t28317: f64, t4142: f64, t4147: f64, t4268: f64, t5636: f64, t6662: f64, t7510: f64, t82120: f64, t82123: f64, t855: f64, t92458: f64, t23237: f64, t28276: f64, t6552: f64, t16662: f64, t6553: f64, t6554: f64, t23164: f64, t16968: f64, t87052: f64, t87053: f64, t16887: f64, t87057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98237, t98239, t98248, t98251, t98253) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2220(t23204, t28298, t81640, t225, t28442, t22986, t23270, t25191, t4300, t25192, t86873, t5544, t857);
        let t98258 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2221(t22986, t23270, t865, t98253, t1528, t2597, t28311, t866, t86951, t86968, t86988, t92432, t98234, t98237, t98239, t98248, t98251);
        let (t98264, t98277, t98279, t98291, t98305) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2222(t1527, t22986, t23270, t86849, t4272, t86969, t1520, t254, t25038, t25039, t4119, t1880, t7488, t87782);
        let t98309 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2223(t10110, t17056, t25168, t25169, t25233, t25330, t259, t2713, t28317, t4142, t4147, t4268, t5636, t6662, t7510, t82120, t82123, t855, t92458, t98291, t98305);
        let (t98315, t98319, t98322, t98325, t98328) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2224(t23237, t28276, t6552, t16662, t6553, t6554, t23164, t23204, t16968, t87052, t87053, t16887, t87057);
    (t98258, t98264, t98277, t98279, t98309, t98315, t98319, t98322, t98325, t98328)
}
