//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2521/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2521(t11270: f64, t4740: f64, t11274: f64, t1657: f64, t11278: f64, t1671: f64, t43954: f64, t11180: f64, t4782: f64, t14914: f64, t3259: f64, t1254: f64, t15834: f64, t3640: f64, t4700: f64, t50816: f64, t50818: f64, t50821: f64, t51111: f64, t51113: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51119 = 1.0_f64 * t4740 * t11270;
    let t51120 = t1657 * t11274;
    let t51122 = 0.51726012919273400301e3_f64 * t51120 * t11278;
    let t51124 = 1.0_f64 * t43954 * t1671;
    let t51126 = 3.0_f64 * t11180 * t4782;
    let t51128 = 3.0_f64 * t3259 * t14914;
    let t51129 = -3.0_f64 * t1254 * t15834 * t3640 * t4700 - t50816 - t50818 - t50821 - t51111 - t51113 + t51119 + t51122 + t51124 + t51126 + t51128;
    (t51119, t51122, t51124, t51126, t51128, t51129)
}
