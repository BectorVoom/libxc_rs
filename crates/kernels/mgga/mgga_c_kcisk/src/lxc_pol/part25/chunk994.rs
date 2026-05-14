//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 994/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk994<F: Float>(t1705: F, t7088: F, t2404: F, t4908: F, t1746: F, t4929: F, t7180: F, t4948: F, t4957: F, t17402: F, t10937: F, t10941: F, t10944: F, t10947: F, t11091: F, t11092: F, t17399: F, t17405: F, t17408: F, t17412: F) -> (F, F, F, F, F) {
    let t17562 = t7088 * t1705;
    let t17567 = t2404 * t4908;
    let t17570 = t1746 * t4929;
    let t17571 = t7180 * t17570;
    let t17574 = t4957 * t4948;
    let t17575 = t7180 * t17574;
    let t17594 = 0.13418888888888888889e0 * t17402;
    let t17598 = -0.26837777777777777778e0 * t10937 + 0.67094444444444444447e-1 * t10941 - 0.20128333333333333334e0 * t10944 + 0.10064166666666666667e0 * t10947 - t11091 - t11092 - 0.40256666666666666667e0 * t17399 + t17594 - 0.20128333333333333333e0 * t17405 - 0.33547222222222222222e0 * t17408 + 0.80513333333333333333e0 * t17412;
    (t17562, t17567, t17571, t17575, t17598)
}
