//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2986/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2986<F: Float>(t10403: F, t10422: F, t18035: F, t10904: F, t10937: F, t13995: F, t14033: F, t14037: F, t14085: F, t14174: F, t17734: F, t17988: F, t18021: F, t18036: F, t2960: F, t3070: F, t3071: F, t3121: F, t42505: F, t43114: F, t4590: F, t4644: F, t49972: F, t49987: F, t49989: F, t49993: F, t5681: F) -> F {
    let t62418 = t10403 * t10422 * t18035;
    let t62427 = -t10904 * t17734 / F::cast_from(72.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t4644 * t14174 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t14085 * t4590 - t49972 / F::cast_from(324.0_f64) - t43114 / F::cast_from(10368.0_f64) + t13995 * t14033 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t13995 * t14037 - t3070 * t3071 * t5681 * t3121 / F::cast_from(2304.0_f64) - t42505 * t18036 / F::cast_from(216.0_f64) + t62418 / F::cast_from(1728.0_f64) - t10937 * t18021 / F::cast_from(432.0_f64) - t49987 / F::cast_from(216.0_f64) - t49989 / F::cast_from(216.0_f64) - t49993 / F::cast_from(3456.0_f64) - t2960 * t17988 / F::cast_from(9.0_f64);
    t62427
}
