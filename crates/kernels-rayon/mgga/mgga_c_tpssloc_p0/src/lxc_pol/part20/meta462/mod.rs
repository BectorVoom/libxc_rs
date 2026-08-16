//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1926;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta462(t3536: f64, t4997: f64, t248: f64, t3570: f64, t5012: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64, t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11665: f64, t1218: f64, t1232: f64, t15470: f64, t15474: f64, t15478: f64, t15484: f64, t15488: f64, t3511: f64, t3518: f64, t3527: f64, t3577: f64, t3587: f64, t4954: f64, t5005: f64, t5024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15490, t15492, t15494, t15495, t15498, t15501) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1926(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023, t1742, t3036);
        let (t15502, t15503, t15506, t15507, t15512) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1927(t15501, t3503, t3500, t1210, t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
    (t15492, t15495, t15498, t15502, t15503, t15506, t15507, t15512)
}
