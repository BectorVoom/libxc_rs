//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1432/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1432(t104740: f64, t104749: f64, t104818: f64, t106758: f64, t106800: f64, t106813: f64, t106816: f64, t106842: f64, t106849: f64, t1409: f64, t1860: f64, t1864: f64, t20217: f64, t20234: f64, t20245: f64, t2109: f64, t2110: f64, t21510: f64, t24498: f64, t24514: f64, t26016: f64, t27298: f64, t27356: f64, t27956: f64, t29474: f64, t5392: f64, t5398: f64, t56: f64, t67: f64, t7246: f64, t7251: f64, t7445: f64, t7974: f64, t83803: f64, t85539: f64, t90137: f64, t96157: f64, t96443: f64) -> f64 {
    let t108983 = -5.0_f64 * t26016 * t104740 - 15.0_f64 * t24514 * t106758 + 30.0_f64 * t90137 * t104749 - 10.0_f64 * t96443 * t27298 - t1860 * (-1232.0_f64 / 27.0_f64 * t20245 * t56 - 220.0_f64 / 9.0_f64 * t104818 * t1409 - 20.0_f64 / 9.0_f64 * t96157 * t5392 + 20.0_f64 / 3.0_f64 * t27356 * t5398 + 5.0_f64 / 108.0_f64 * t85539 * t20234 + 5.0_f64 / 6.0_f64 * t24498 * t21510 - 5.0_f64 / 6.0_f64 * t7251 * t20217 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t29474 * t7445 / 2.0_f64 - t1860 * t7974 * t27956 / 2.0_f64 - t1860 * t2109 * t106800 / 6.0_f64 + t106816 * t2110 + 5.0_f64 / 2.0_f64 * t7246 * t106813 + 5.0_f64 / 2.0_f64 * t7246 * t106842 + 5.0_f64 / 6.0_f64 * t7246 * t106849;
    t108983
}
