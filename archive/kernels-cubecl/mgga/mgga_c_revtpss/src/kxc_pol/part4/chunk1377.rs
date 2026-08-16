//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1377/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1377<F: Float>(t16756: F, t5341: F, t3720: F, t12916: F, t5342: F, t5340: F, t12702: F, t5330: F, t12744: F, t1222: F, t1266: F, t12853: F, t17401: F, t17405: F, t17412: F, t17417: F, t3689: F, t3694: F, t3723: F, t5335: F, t5343: F, t5373: F) -> F {
    let t17419 = t16756 * t5341;
    let t17420 = t3720 * t17419;
    let t17423 = t12916 * t5342;
    let t17425 = F::cast_from(0.57165357490759649296e-3_f64) * t5340 * t17423;
    let t17426 = t12702 * t5330;
    let t17429 = t12744 * t5330;
    let t17432 = -F::cast_from(0.42874018118069736972e-3_f64) * t17401 * t3723 - t1222 * t17405 / F::cast_from(288.0_f64) + t5373 * t3689 / F::cast_from(108.0_f64) + t5373 * t3694 / F::cast_from(54.0_f64) + F::cast_from(0.15244095330869239812e-2_f64) * t17412 * t1266 + F::cast_from(0.31758531939310916276e-4_f64) * t17417 + F::cast_from(0.85748036236139473944e-3_f64) * t5340 * t17420 + t12853 + t17425 + F::cast_from(0.85748036236139473944e-3_f64) * t17426 * t5343 - F::cast_from(0.42874018118069736972e-3_f64) * t17429 * t5335;
    t17432
}
