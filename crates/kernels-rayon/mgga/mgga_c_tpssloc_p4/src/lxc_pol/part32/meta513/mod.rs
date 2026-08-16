//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1841;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1842;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta513(t26309: f64, t5252: f64, t22833: f64, t5293: f64, t5303: f64, t1351: f64, t16311: f64, t3788: f64, t6936: f64, t16306: f64, t550: f64, t1339: f64, t22856: f64, t22859: f64, t22860: f64, t22864: f64, t22868: f64, t26306: f64, t22766: f64, t22780: f64, t22798: f64, t22805: f64, t22820: f64, t22826: f64, t26231: f64, t26234: f64, t26236: f64, t26238: f64, t26240: f64, t26246: f64, t26249: f64, t26251: f64, t26280: f64, t26286: f64, t26290: f64, t26293: f64, t26295: f64, t26299: f64, t26303: f64, t539: f64, t1887: f64, t22839: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1840(t26309, t5252, t22833, t5293, t5303, t1351, t16311, t3788, t6936, t16306, t550, t1339);
        let t26326 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1841(t26323, t6936, t22856, t22859, t22860, t22864, t22868, t26306, t26310, t26312, t26314, t26320);
        let t26328 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1842(t22766, t22780, t22798, t22805, t22820, t22826, t26231, t26234, t26236, t26238, t26240, t26246, t26249, t26251, t26280, t26286, t26290, t26293, t26295, t26299, t26303, t26326);
        let (t26329, t26331) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1843(t26328, t539, t1887, t22839);
    (t26318, t26319, t26322, t26323, t26328, t26329, t26331)
}
