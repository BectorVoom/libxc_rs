//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1044/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1044<F: Float>(t128296: F, t2039: F, t33211: F, t7801: F, t28017: F, t88: F, t33596: F, t28951: F, t8601: F, t102386: F, t1873: F, t122617: F, t126127: F, t126132: F, t128371: F, t128555: F, t1458: F, t24999: F, t31532: F, t33085: F, t5493: F) -> F {
    let t128953 = F::cast_from(4.0_f64) * t128296 * t2039;
    let t128955 = F::cast_from(4.0_f64) * t33211 * t7801;
    let t128956 = t88 * t28017;
    let t128958 = F::cast_from(2.0_f64) * t128956 * t2039;
    let t128960 = F::cast_from(4.0_f64) * t33596 * t7801;
    let t128962 = F::cast_from(2.0_f64) * t8601 * t28951;
    let t128968 = F::cast_from(2.0_f64) * t102386 * t1873;
    let t128970 = F::cast_from(4.0_f64) * t122617 * t1458 + F::cast_from(4.0_f64) * t126127 * t2039 + F::cast_from(2.0_f64) * t126132 * t2039 + F::cast_from(4.0_f64) * t24999 * t7801 + F::cast_from(2.0_f64) * t31532 * t5493 + F::cast_from(4.0_f64) * t33085 * t7801 + t128371 + F::cast_from(2.0_f64) * t128555 + t128953 + t128955 + t128958 + t128960 + t128962 + t128968;
    t128970
}
