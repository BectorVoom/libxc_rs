//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 690/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk690<F: Float>(t13387: F, t1429: F, t11426: F, t6590: F, t3516: F, t6508: F, t2365: F, t4391: F, t123: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t13388 = t1429 * t13387;
    let t13389 = F::cast_from(0.14896037479937677779e-1_f64) * t13388;
    let t13390 = t11426 * t6590;
    let t13392 = t6508 * t3516;
    let t13393 = t2365 * t13392;
    let t13394 = t4391 * t13393;
    let t13395 = F::cast_from(0.29792074959875355558e-1_f64) * t13394;
    let t13396 = t3516 * t123;
    let t13397 = t13396 * t883;
    (t13389, t13390, t13392, t13393, t13395, t13396, t13397)
}
