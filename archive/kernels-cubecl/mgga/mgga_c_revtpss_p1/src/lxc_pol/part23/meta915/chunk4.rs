//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2953/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2953<F: Float>(t11409: F, t11507: F, t15104: F, t15266: F, t15406: F, t19263: F, t19266: F, t19269: F, t23706: F, t311: F, t52812: F, t6205: F, t77598: F, t78319: F, t78322: F, t78325: F, t78328: F, t78332: F, t78335: F, t78339: F, t78342: F, t78375: F, t78394: F, t953: F, t972: F) -> F {
    let t78398 = -t78319 + t78322 + t78325 + t78328 - t78332 - t78335 - t78339 - t78342 + F::cast_from(18.0_f64) * t15406 * t19263 - F::cast_from(12.0_f64) * t15104 * t19266 - F::cast_from(0.57895126195293126241e3_f64) * t52812 * t19269 - F::cast_from(24.0_f64) * t11409 * t23706 * t953 - F::cast_from(0.19751673498613801407e-1_f64) * t77598 + F::cast_from(0.30762056574649219973e4_f64) * t11507 * t6205 * t15266 * t972 - F::cast_from(0.310907e-1_f64) * (t78375 + t78394) * t311;
    t78398
}
