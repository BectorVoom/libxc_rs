//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1412/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1412<F: Float>(t1382: F, t1651: F, t3718: F, t12035: F, t4342: F, t4339: F, t12148: F, t605: F, t12032: F, t12323: F, t747: F, t1960: F, t1961: F, t2208: F, t31458: F, t31461: F, t31463: F, t31465: F, t31470: F, t31472: F, t31474: F, t31476: F, t32708: F, t32713: F, t32715: F, t32716: F, t32719: F, t32720: F, t3749: F, t38456: F, t5559: F, t841: F) -> (F, F, F, F, F, F) {
    let t38872 = F::cast_from(2.0_f64) * t1382 * t3718 * t1651;
    let t38874 = F::cast_from(4.0_f64) * t4342 * t12035;
    let t38876 = t4339 * t3718;
    let t38880 = F::cast_from(4.0_f64) * t1382 * t12148 * t605;
    let t38881 = t12032 * t1651;
    let t38885 = t12323 * t747;
    let t38891 = F::cast_from(2.0_f64) * t1960 * t2208 * t3749 - F::cast_from(6.0_f64) * t1961 * t3749 * t5559 - F::cast_from(2.0_f64) * t38885 * t841 + t31458 + t31461 + t31463 - t31465 + t31470 - t31472 + t31474 - t31476 + t32708 - t32713 + t32715 - t32716 - t32719 - t32720 - t38456;
    (t38872, t38874, t38876, t38880, t38881, t38891)
}
