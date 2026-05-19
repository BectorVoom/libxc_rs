//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 901/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk901<F: Float>(t39677: F, t39679: F, t39681: F, t12837: F, t6305: F, t2268: F, t2440: F, t3340: F, t10223: F, t894: F, t1063: F, t12814: F, t12971: F, t42580: F, t42582: F, t42584: F, t42588: F, t42591: F, t42594: F, t42597: F, t42601: F, t42602: F, t42603: F, t448: F, t535: F) -> F {
    let t42604 = F::cast_from(0.23712505529730124666e-2_f64) * t39677;
    let t42605 = F::cast_from(0.47425011059460249332e-2_f64) * t39679;
    let t42606 = F::cast_from(0.71137516589190373998e-2_f64) * t39681;
    let t42607 = t6305 * t12837;
    let t42610 = t2268 * t2440 * t3340;
    let t42613 = t2268 * t894 * t10223;
    let t42621 = -t42580 + t42582 + F::cast_from(0.47425011059460249332e-2_f64) * t42584 - t42588 - t42591 - t42594 + t42597 + t42601 + t42602 - t42603 + t42604 + t42605 - t42606 + F::cast_from(0.56910013271352299198e-1_f64) * t42607 + F::cast_from(0.56910013271352299198e-1_f64) * t42610 + F::cast_from(0.56910013271352299198e-1_f64) * t42613 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t12971 * t448 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t12814;
    t42621
}
