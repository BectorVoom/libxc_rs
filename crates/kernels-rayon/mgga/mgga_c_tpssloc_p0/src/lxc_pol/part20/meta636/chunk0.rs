//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2337/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2337(t10481: f64, t23508: f64, t10469: f64, t1603: f64, t11058: f64, t1625: f64, t11045: f64, t11064: f64, t1058: f64, t1060: f64, t10857: f64, t11028: f64, t11034: f64, t11040: f64, t11046: f64, t11048: f64, t11049: f64, t11061: f64, t11067: f64, t14608: f64, t14622: f64, t14654: f64, t3200: f64, t43480: f64, t43536: f64, t4669: f64, t4674: f64, t4677: f64, t4685: f64) -> (f64, f64, f64, f64) {
    let t47819 = t23508 * t10481;
    let t47840 = t1603 * t10469;
    let t47841 = t47840 * t11058;
    let t47844 = t1625 * t10481;
    let t47853 = t47840 * t11045;
    let t47857 = t47840 * t11064;
    let t47867 = t1058 * t1060 * t10857 * t1625 + t11046 * t11048 * t47844 - 3.0_f64 * t14622 * t3200 * t4677 + t11028 * t4669 + 6.0_f64 * t11034 * t14654 - 3.0_f64 * t11040 * t14608 + t11049 * t47853 + 6.0_f64 * t11061 * t47841 - 6.0_f64 * t11067 * t47857 + 6.0_f64 * t43480 * t4674 - 3.0_f64 * t43536 * t4685;
    (t47819, t47840, t47844, t47867)
}
