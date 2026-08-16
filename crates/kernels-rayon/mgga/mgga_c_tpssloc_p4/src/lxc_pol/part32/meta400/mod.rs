//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1513;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1514;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta400(t17375: f64, t17449: f64, t17516: f64, t17558: f64, t300: f64, t2940: f64, t5808: f64, t10629: f64, t5774: f64, t10632: f64, t950: f64, t959: f64, t225: f64, t5849: f64, t1603: f64, t4657: f64, t1634: f64, t4693: f64, t3174: f64, t5851: f64, t17183: f64, t977: f64, t17178: f64, t2979: f64, t17161: f64, t10214: f64, t17152: f64, t1040: f64, t5904: f64, t248: f64, t3101: f64, t5867: f64, t1020: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t1046: f64, t13750: f64, t13758: f64, t13767: f64, t13946: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17561, t17563, t17568) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1513(t17375, t17449, t17516, t17558, t300, t2940, t5808, t10629, t5774, t10632, t950, t959);
        let (t17575, t17579, t17583, t17588, t17593, t17596) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1514(t225, t5849, t1603, t4657, t1634, t4693, t3174, t5851, t17183, t977, t17178, t2979);
        let t17614 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1515(t17161, t2979, t10214, t17152, t1040, t5904, t248, t3101, t5867, t1020, t10372, t10377, t10381, t10385, t1046, t13750, t13758, t13767, t13946, t17593, t17596, t973);
    (t17561, t17563, t17568, t17575, t17579, t17583, t17588, t17614)
}
