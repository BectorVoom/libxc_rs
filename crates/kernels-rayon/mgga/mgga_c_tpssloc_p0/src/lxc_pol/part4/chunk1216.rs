//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1216/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1216(t19862: f64, t19899: f64, t19939: f64, t20007: f64, t553: f64, t5287: f64, t5335: f64, t1352: f64, t19739: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t16060: f64, t1814: f64, t1838: f64, t1840: f64, t19756: f64, t19761: f64, t19763: f64, t19805: f64, t19810: f64, t19813: f64, t19815: f64, t5230: f64, t5234: f64, t5339: f64, t5341: f64, t5344: f64, t5345: f64, t5351: f64, t544: f64, t564: f64, t6378: f64, t6458: f64) -> (f64, f64) {
    let t20009 = t19862 + t19899 + t19939 + t20007;
    let t20010 = t553 * t20009;
    let t20014 = t5335 * t5287;
    let t20018 = t19739 * t1352;
    let t20021 = t1332 * t6458 - 2.0_f64 * t1336 * t19756 - t1336 * t19813 - t1381 * t19815 + t1383 * t6378 - 2.0_f64 * t16060 * t1838 + 2.0_f64 * t1814 * t5351 + 2.0_f64 * t1840 * t5230 - t19761 * t5344 - t19763 * t5344 + t19805 * t564 - 2.0_f64 * t19810 * t5345 + t20010 * t544 - 2.0_f64 * t20014 * t5344 - 2.0_f64 * t20018 * t5344 - 2.0_f64 * t5234 * t5339 - 2.0_f64 * t5234 * t5341;
    (t20009, t20021)
}
