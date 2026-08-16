//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1979;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1980;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1981;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1982;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta439(t3536: f64, t4997: f64, t248: f64, t3570: f64, t5012: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64, t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11665: f64, t1218: f64, t1232: f64, t15470: f64, t15474: f64, t15478: f64, t15484: f64, t15488: f64, t3511: f64, t3518: f64, t3527: f64, t3577: f64, t3587: f64, t4954: f64, t5005: f64, t5024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15490, t15492, t15494, t15495) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1979(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018);
        let t15498 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1980(t1202, t5023);
        let (t15501, t15502, t15503) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1981(t1742, t3036, t3503, t3500);
        let (t15506, t15507) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1982(t1210, t15501, t3500);
        let t15512 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1983(t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t15503, t15507, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
    (t15490, t15492, t15494, t15495, t15498, t15502, t15503, t15506, t15507, t15512)
}
