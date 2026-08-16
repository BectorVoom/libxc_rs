//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2039;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2040;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta466(t16148: f64, t3870: f64, t820: f64, t1799: f64, t3719: f64, t3799: f64, t5289: f64, t11984: f64, t15876: f64, t15878: f64, t15880: f64, t15887: f64, t15888: f64, t15889: f64, t15891: f64, t15894: f64, t15896: f64, t15898: f64, t15910: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t15911: f64, t15915: f64, t15916: f64, t15917: f64, t15923: f64, t15927: f64, t15928: f64, t9789: f64, t9793: f64, t9797: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t12118: f64, t12123: f64, t15970: f64, t15972: f64, t15973: f64, t15974: f64, t15975: f64, t15976: f64, t15978: f64, t9820: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16150, t16153, t16155, t16159, t16160) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2039(t16148, t3870, t820, t1799, t3719, t3799, t5289, t11984, t15876, t15878, t15880, t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15910, t9457, t9476, t9484, t9780);
        let t16161 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2040(t12044, t12048, t12057, t12059, t12087, t12094, t15911, t15915, t15916, t15917, t15923, t15927, t15928, t9789, t9793, t9797);
        let t16163 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2041(t12103, t12105, t12109, t12114, t12116, t12118, t12123, t15970, t15972, t15973, t15974, t15975, t15976, t15978, t9820, t9824);
    (t16150, t16153, t16155, t16159, t16160, t16161, t16163)
}
