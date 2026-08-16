//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1044/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1044<F: Float>(t113875: F, t115896: F, t641: F, t1862: F, t113876: F, t31680: F, t9239: F, t113864: F, t115833: F, t113871: F, t115863: F, t115866: F, t115871: F, t115873: F, t115877: F, t115880: F, t115884: F, t115889: F, t115891: F, t115895: F, t31672: F, t31677: F, t31681: F, t31684: F, t31693: F, t7026: F, t8512: F) -> F {
    let t115898 = t113875 * t115896 * t641;
    let t115903 = t113875 * t1862;
    let t115904 = t115903 * t113876;
    let t115907 = t9239 * t31680;
    let t115908 = t115833 * t113864;
    let t115911 = -F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8512 * t115863 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t115866 * t31677 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31672 * t31693 - F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t115871 * t115873 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t115877 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t7026 * t115880 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31681 * t115884 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t115889 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t115891 * t31684 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t115895 * t115898 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31681 * t113871 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t31681 * t115904 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t115907 * t115908;
    t115911
}
