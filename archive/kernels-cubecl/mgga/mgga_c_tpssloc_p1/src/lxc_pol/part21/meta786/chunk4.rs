//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2730/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2730<F: Float>(t12250: F, t5286: F, t12240: F, t12267: F, t1336: F, t1351: F, t16033: F, t16044: F, t16047: F, t16055: F, t16206: F, t19660: F, t19668: F, t19732: F, t19739: F, t19745: F, t19748: F, t19752: F, t19810: F, t20018: F, t3777: F, t3851: F, t3901: F, t3909: F, t5334: F, t5335: F, t5344: F, t54976: F, t6378: F, t6448: F) -> F {
    let t57568 = t12250 * t5286;
    let t57597 = -F::cast_from(24.0_f64) * t1351 * t16047 * t5335 * t57568 + F::cast_from(4.0_f64) * t12240 * t19739 * t5334 - F::cast_from(2.0_f64) * t1336 * t19732 * t3901 - F::cast_from(2.0_f64) * t16206 * t5335 * t5344 - t19660 * t3851 * t5344 + F::cast_from(2.0_f64) * t12267 * t6448 - F::cast_from(4.0_f64) * t16033 * t20018 - F::cast_from(2.0_f64) * t16044 * t19810 + F::cast_from(12.0_f64) * t16055 * t19748 + F::cast_from(4.0_f64) * t19668 * t3777 - F::cast_from(12.0_f64) * t19745 * t54976 - F::cast_from(4.0_f64) * t19752 * t3777 + t3909 * t6378;
    t57597
}
