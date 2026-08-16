//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1707;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1708;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1709;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1710;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta413(t3793: f64, t3805: f64, t5301: f64, t3802: f64, t5234: f64, t3788: f64, t836: f64, t1336: f64, t5252: f64, t3777: f64, t5245: f64, t12419: f64, t12420: f64, t5249: f64, t12215: f64, t12335: f64, t12340: f64, t12346: f64, t12356: f64, t12358: f64, t12366: f64, t12386: f64, t12388: f64, t12395: f64, t12429: f64, t16366: f64, t16370: f64, t16379: f64, t16383: f64, t16387: f64, t3803: f64, t3809: f64, t5246: f64, t5303: f64, t16253: f64, t16319: f64, t16361: f64, t553: f64, t3901: f64, t5287: f64, t1352: f64, t16036: f64, t3856: f64, t5348: f64, t1834: f64, t3787: f64, t1380: f64, t16206: f64, t12267: f64, t1383: f64, t16133: f64, t16136: f64, t1814: f64, t1838: f64, t1840: f64, t3773: f64, t3898: f64, t3905: f64, t3907: f64, t3909: f64, t5230: f64, t5339: f64, t5341: f64, t5344: f64, t544: f64, t16131: f64, t1378: f64, t225: f64, t5319: f64, t1372: f64, t5210: f64, t12030: f64, t12444: f64, t1375: f64, t1386: f64, t16022: f64, t16028: f64, t16030: f64, t1843: f64, t3758: f64, t3889: f64, t3912: f64, t5215: f64, t5321: f64, t5354: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16391, t16394, t16400, t16401, t16405) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1707(t3793, t3805, t5301, t3802, t5234, t3788, t836, t1336, t5252, t3777, t5245, t12419, t12420, t5249);
        let t16411 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1708(t12215, t12335, t12340, t12346, t12356, t12358, t12366, t12386, t12388, t12395, t12429, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16400, t16401, t16405, t3803, t3809, t5246, t5252, t5303);
        let (t16413, t16414, t16416, t16419, t16423, t16428) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1709(t16253, t16319, t16361, t16411, t553, t3901, t5287, t1352, t16036, t3856, t5348, t1834, t3787);
        let t16435 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1710(t16428, t3793, t1380, t16206, t12267, t1336, t1383, t16133, t16136, t16414, t16416, t16419, t16423, t1814, t1838, t1840, t3773, t3777, t3898, t3905, t3907, t3909, t5230, t5234, t5339, t5341, t5344, t544);
        let (t16436, t16437, t16439, t16448, t16451) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1711(t16131, t16435, t1378, t225, t5319, t1372, t5210, t12030, t12444, t1375, t1386, t16022, t16028, t16030, t1843, t3758, t3889, t3912, t5215, t5321, t5354, t568);
    (t16391, t16405, t16413, t16419, t16436, t16437, t16439, t16448, t16451)
}
