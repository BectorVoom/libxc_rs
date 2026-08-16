//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1138/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1138<F: Float>(t533: F, t7939: F, t1390: F, t2095: F, t5161: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t2096: F, t4028: F, t510: F, t574: F, t652: F, t7042: F, t7458: F, t7685: F, t7787: F, t7796: F, t7802: F, t7806: F, t7890: F, t7900: F, t7904: F) -> (F, F, F, F) {
    let t7940 = t533 * t7939;
    let t7941 = t7940 * t1390;
    let t7943 = t2095 * t5161;
    let t7945 = -t113 * t7890 - t1442 * t2075 - F::cast_from(2.0_f64) * t1459 * t7042 - t1774 * t2036 + t1849 * t2079 + F::cast_from(3.0_f64) * t1983 * t7904 + t1983 * t7941 - t1983 * t7943 - F::cast_from(2.0_f64) * t2040 * t4028 - F::cast_from(2.0_f64) * t2040 * t7458 + t2096 * t7685 - t510 * t7787 + t574 * t7900 - F::cast_from(2.0_f64) * t652 * t7796 - F::cast_from(2.0_f64) * t652 * t7802 - F::cast_from(2.0_f64) * t652 * t7806;
    (t7940, t7941, t7943, t7945)
}
