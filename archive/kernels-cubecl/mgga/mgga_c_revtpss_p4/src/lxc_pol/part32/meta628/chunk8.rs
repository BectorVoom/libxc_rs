//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2017/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2017<F: Float>(t103286: F, t106030: F, t106033: F, t106035: F, t106037: F, t106040: F, t106042: F, t106044: F, t106046: F, t106048: F, t106050: F, t106053: F, t99013: F) -> F {
    let t110406 = t103286 - F::cast_from(0.57165357490759649296e-4_f64) * t106030 + F::cast_from(0.28582678745379824648e-4_f64) * t106033 + F::cast_from(0.43366402397256813419e-2_f64) * t99013 - F::cast_from(0.34299214494455789578e-2_f64) * t106035 - F::cast_from(0.2032800112371413129e-3_f64) * t106037 + F::cast_from(0.28582678745379824648e-4_f64) * t106040 + F::cast_from(0.40015750243531754507e-2_f64) * t106042 - F::cast_from(0.34299214494455789578e-1_f64) * t106044 - F::cast_from(0.13719685797782315831e-1_f64) * t106046 - F::cast_from(0.50820002809285328225e-4_f64) * t106048 + F::cast_from(0.10164000561857065645e-3_f64) * t106050 - F::cast_from(0.22866142996303859718e-3_f64) * t106053;
    t110406
}
