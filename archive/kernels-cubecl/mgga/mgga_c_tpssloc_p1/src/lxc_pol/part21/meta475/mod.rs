//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2062;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2063;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta475<F: Float>(t16398: F, t5252: F, t3777: F, t5245: F, t12419: F, t12420: F, t5249: F, t12215: F, t12335: F, t12340: F, t12346: F, t12356: F, t12358: F, t12366: F, t12386: F, t12388: F, t12395: F, t12429: F, t16366: F, t16370: F, t16379: F, t16383: F, t16387: F, t16391: F, t16394: F, t3803: F, t3809: F, t5246: F, t5303: F, t16253: F, t16319: F, t16361: F, t553: F, t3901: F, t5287: F, t1352: F, t16036: F, t3856: F, t5348: F, t1834: F, t3787: F, t3793: F, t1380: F, t16206: F, t12267: F, t1336: F, t1383: F, t16133: F, t16136: F, t1814: F, t1838: F, t1840: F, t3773: F, t3898: F, t3905: F, t3907: F, t3909: F, t5230: F, t5234: F, t5339: F, t5341: F, t5344: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16400, t16401) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2061::<F>(t16398, t5252, t3777, t5245);
        let (t16405, t16411) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2062::<F>(t12419, t12420, t5249, t12215, t12335, t12340, t12346, t12356, t12358, t12366, t12386, t12388, t12395, t12429, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16400, t16401, t3803, t3809, t5246, t5252, t5303);
        let (t16413, t16414, t16416, t16419, t16423, t16428) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2063::<F>(t16253, t16319, t16361, t16411, t553, t3901, t5287, t1352, t16036, t3856, t5348, t1834, t3787);
        let (t16429, t16433, t16435) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2064::<F>(t16428, t3793, t1380, t16206, t12267, t1336, t1383, t16133, t16136, t16414, t16416, t16419, t16423, t1814, t1838, t1840, t3773, t3777, t3898, t3905, t3907, t3909, t5230, t5234, t5339, t5341, t5344, t544);
    (t16400, t16401, t16405, t16413, t16414, t16416, t16419, t16423, t16429, t16433, t16435)
}
