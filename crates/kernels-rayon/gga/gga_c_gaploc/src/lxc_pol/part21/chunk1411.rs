//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1411/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1411(t3718: f64, t4339: f64, t12148: f64, t1382: f64, t605: f64, t12032: f64, t1651: f64, t12323: f64, t747: f64, t1960: f64, t1961: f64, t2208: f64, t31458: f64, t31461: f64, t31463: f64, t31465: f64, t31470: f64, t31472: f64, t31474: f64, t31476: f64, t32708: f64, t32713: f64, t32715: f64, t32716: f64, t32719: f64, t32720: f64, t3749: f64, t38456: f64, t5559: f64, t841: f64) -> (f64, f64, f64, f64) {
    let t38876 = t4339 * t3718;
    let t38880 = 4.0_f64 * t1382 * t12148 * t605;
    let t38881 = t12032 * t1651;
    let t38885 = t12323 * t747;
    let t38891 = 2.0_f64 * t1960 * t2208 * t3749 - 6.0_f64 * t1961 * t3749 * t5559 - 2.0_f64 * t38885 * t841 + t31458 + t31461 + t31463 - t31465 + t31470 - t31472 + t31474 - t31476 + t32708 - t32713 + t32715 - t32716 - t32719 - t32720 - t38456;
    (t38876, t38880, t38881, t38891)
}
