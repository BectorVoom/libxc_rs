//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 699/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk699<F: Float>(t13415: F, t1572: F, t12940: F, t12944: F, t12946: F, t13385: F, t13389: F, t13390: F, t13395: F, t13399: F, t13405: F, t13409: F, t13412: F, t597: F) -> F {
    let t13417 = F::new(0.71500979903700853338e0) * t1572 * t13415;
    let t13418 = t13385 - t13389 - F::new(0.44688112439813033338e-1) * t13390 + t13395 + F::new(0.95857314884801874192e0) * t13399 - t13405 - F::new(0.63904876589867916128e-1) * t12940 - F::new(0.59584149919750711116e-1) * t12944 + F::new(0.59584149919750711116e-1) * t12946 + F::new(0.14300195980740170668e1) * t1572 * t13409 + F::new(0.23005755572352449806e2) * t597 * t13412 + t13417;
    t13418
}
