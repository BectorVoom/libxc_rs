//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 622/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk622<F: Float>(t13392: F, t2365: F, t4391: F, t123: F, t3516: F, t883: F) -> (F, F, F, F) {
    let t13393 = t2365 * t13392;
    let t13394 = t4391 * t13393;
    let t13395 = 0.29792074959875355558e-1 * t13394;
    let t13396 = t3516 * t123;
    let t13397 = t13396 * t883;
    (t13393, t13395, t13396, t13397)
}
