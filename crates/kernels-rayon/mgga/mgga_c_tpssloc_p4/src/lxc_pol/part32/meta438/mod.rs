//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1681;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta438(t19862: f64, t19899: f64, t19939: f64, t20007: f64, t553: f64, t5287: f64, t5335: f64, t1352: f64, t19739: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t16060: f64, t1814: f64, t1838: f64, t1840: f64, t19756: f64, t19761: f64, t19763: f64, t19805: f64, t19810: f64, t19813: f64, t19815: f64, t5230: f64, t5234: f64, t5339: f64, t5341: f64, t5344: f64, t5345: f64, t5351: f64, t544: f64, t564: f64, t6378: f64, t6458: f64, t19755: f64, t1378: f64, t1385: f64, t6460: f64, t3887: f64, t225: f64, t6364: f64, t539: f64, t1375: f64, t1386: f64, t16030: f64, t16439: f64, t1843: f64, t19635: f64, t19644: f64, t19648: f64, t3882: f64, t5321: f64, t5326: f64, t5354: f64, t568: f64, t6461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20009, t20014, t20018, t20021) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1681(t19862, t19899, t19939, t20007, t553, t5287, t5335, t1352, t19739, t1332, t1336, t1381, t1383, t16060, t1814, t1838, t1840, t19756, t19761, t19763, t19805, t19810, t19813, t19815, t5230, t5234, t5339, t5341, t5344, t5345, t5351, t544, t564, t6378, t6458);
        let (t20022, t20023, t20025, t20026, t20029, t20032, t20034) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1682(t19755, t20021, t1378, t1385, t6460, t3887, t225, t6364, t20009, t539, t1375, t1386, t16030, t16439, t1843, t19635, t19644, t19648, t3882, t5321, t5326, t5354, t568, t6461);
    (t20009, t20014, t20018, t20022, t20023, t20025, t20026, t20029, t20032, t20034)
}
