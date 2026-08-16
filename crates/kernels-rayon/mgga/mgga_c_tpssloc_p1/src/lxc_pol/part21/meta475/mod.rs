//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2062;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2063;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta475(t16398: f64, t5252: f64, t3777: f64, t5245: f64, t12419: f64, t12420: f64, t5249: f64, t12215: f64, t12335: f64, t12340: f64, t12346: f64, t12356: f64, t12358: f64, t12366: f64, t12386: f64, t12388: f64, t12395: f64, t12429: f64, t16366: f64, t16370: f64, t16379: f64, t16383: f64, t16387: f64, t16391: f64, t16394: f64, t3803: f64, t3809: f64, t5246: f64, t5303: f64, t16253: f64, t16319: f64, t16361: f64, t553: f64, t3901: f64, t5287: f64, t1352: f64, t16036: f64, t3856: f64, t5348: f64, t1834: f64, t3787: f64, t3793: f64, t1380: f64, t16206: f64, t12267: f64, t1336: f64, t1383: f64, t16133: f64, t16136: f64, t1814: f64, t1838: f64, t1840: f64, t3773: f64, t3898: f64, t3905: f64, t3907: f64, t3909: f64, t5230: f64, t5234: f64, t5339: f64, t5341: f64, t5344: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16400, t16401) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2061(t16398, t5252, t3777, t5245);
        let (t16405, t16411) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2062(t12419, t12420, t5249, t12215, t12335, t12340, t12346, t12356, t12358, t12366, t12386, t12388, t12395, t12429, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16400, t16401, t3803, t3809, t5246, t5252, t5303);
        let (t16413, t16414, t16416, t16419, t16423, t16428) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2063(t16253, t16319, t16361, t16411, t553, t3901, t5287, t1352, t16036, t3856, t5348, t1834, t3787);
        let (t16429, t16433, t16435) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2064(t16428, t3793, t1380, t16206, t12267, t1336, t1383, t16133, t16136, t16414, t16416, t16419, t16423, t1814, t1838, t1840, t3773, t3777, t3898, t3905, t3907, t3909, t5230, t5234, t5339, t5341, t5344, t544);
    (t16400, t16401, t16405, t16413, t16414, t16416, t16419, t16423, t16429, t16433, t16435)
}
