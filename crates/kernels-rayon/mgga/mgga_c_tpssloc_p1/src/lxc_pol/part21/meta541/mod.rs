//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2225;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2226;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta541(t11721: f64, t1215: f64, t18300: f64, t4582: f64, t4978: f64, t1222: f64, t6170: f64, t6158: f64, t6165: f64, t11644: f64, t11649: f64, t11719: f64, t11728: f64, t15446: f64, t15448: f64, t15450: f64, t15452: f64, t15503: f64, t15507: f64, t18297: f64, t488: f64, t4974: f64, t4980: f64, t4984: f64, t5005: f64, t5416: f64, t972: f64, t135: f64, t6187: f64, t1174: f64, t4889: f64, t5040: f64, t6183: f64, t6177: f64, t1198: f64, t15484: f64, t15488: f64, t15490: f64, t15494: f64, t15498: f64, t15524: f64, t15550: f64, t15574: f64, t15580: f64, t15737: f64, t1748: f64, t5024: f64, t5030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18301, t18302, t18303, t18306, t18307, t18316) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2225(t11721, t1215, t18300, t4582, t4978, t1222, t6170, t6158, t6165, t11644, t11649, t11719, t11728, t15446, t15448, t15450, t15452, t15503, t15507, t18297, t488, t4974, t4980, t4984, t5005);
        let t18321 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2226(t5416, t972);
        let t18337 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2227(t135, t6187, t1174, t4889, t5040, t6183, t6177, t1198, t15484, t15488, t15490, t15494, t15498, t15524, t15550, t15574, t15580, t15737, t1748, t18321, t4980, t5024, t5030);
    (t18301, t18302, t18303, t18306, t18307, t18316, t18321, t18337)
}
