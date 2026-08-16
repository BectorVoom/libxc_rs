//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1636;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1637;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1638;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta399(t3536: f64, t4997: f64, t248: f64, t3570: f64, t5012: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64, t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11665: f64, t1218: f64, t1232: f64, t15470: f64, t15474: f64, t15478: f64, t15484: f64, t15488: f64, t3511: f64, t3518: f64, t3527: f64, t3577: f64, t3587: f64, t4954: f64, t5005: f64, t5024: f64, t11539: f64, t4724: f64, t1174: f64, t15239: f64, t475: f64, t1214: f64, t3494: f64, t4977: f64, t4582: f64, t3516: f64, t12652: f64, t4987: f64, t12648: f64, t13969: f64, t4983: f64, t3515: f64, t486: f64, t5011: f64, t4978: f64, t11709: f64, t11738: f64, t11814: f64, t11825: f64, t1227: f64, t1737: f64, t1748: f64, t3490: f64, t3506: f64, t3531: f64, t4980: f64, t4989: f64, t5014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15490, t15492, t15494, t15495, t15498, t15501) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1636(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023, t1742, t3036);
        let t15512 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1637(t15501, t3503, t3500, t1210, t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
        let (t15524, t15527, t15531, t15535, t15540) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1638(t11539, t4724, t1174, t15239, t475, t1214, t248, t3494, t4977, t4582, t3516, t12652, t4987);
        let (t15541, t15545, t15548, t15553, t15555, t15558) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1639(t15540, t4582, t12648, t4987, t13969, t4983, t3515, t486, t5011, t4978, t11709, t11738, t11814, t11825, t1213, t1227, t15524, t15527, t15531, t15535, t1737, t1748, t3490, t3506, t3531, t3536, t4980, t4989, t5014, t5024);
    (t15492, t15501, t15512, t15527, t15531, t15535, t15541, t15545, t15548, t15553, t15555, t15558)
}
