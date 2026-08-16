//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2010/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2010<F: Float>(t102344: F, t1458: F, t19534: F, t2039: F, t2314: F, t23938: F, t26114: F, t26117: F, t26977: F, t27170: F, t27188: F, t28007: F, t28951: F, t33234: F, t4072: F, t5113: F, t5493: F, t55943: F, t7042: F, t7056: F, t7676: F, t7801: F, t96657: F) -> F {
    let t102432 = F::cast_from(4.0_f64) * t102344 * t1458 + F::cast_from(2.0_f64) * t19534 * t7042 + F::cast_from(2.0_f64) * t2039 * t55943 + F::cast_from(2.0_f64) * t2039 * t96657 + F::cast_from(2.0_f64) * t2314 * t28951 + F::cast_from(2.0_f64) * t23938 * t5493 + F::cast_from(4.0_f64) * t26114 * t7801 + F::cast_from(4.0_f64) * t26117 * t7801 + F::cast_from(2.0_f64) * t26977 * t5493 + F::cast_from(4.0_f64) * t27170 * t7676 + F::cast_from(4.0_f64) * t27188 * t4072 + F::cast_from(2.0_f64) * t28007 * t7056 + F::cast_from(2.0_f64) * t28951 * t5113 + F::cast_from(4.0_f64) * t33234 * t4072;
    t102432
}
