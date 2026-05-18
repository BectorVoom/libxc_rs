//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 700/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk700<F: Float>(t3684: F, t977: F, t1960: F, t3601: F, t7290: F, t2365: F, t6111: F, t2610: F, t3614: F, t2033: F, t11845: F, t959: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13585 = t3684 * t977;
    let t13587 = F::new(2.0) * t1960 * t13585;
    let t13588 = t7290 * t3601;
    let t13589 = t2365 * t13588;
    let t13590 = t6111 * t13589;
    let t13591 = F::new(0.29792074959875355558e-1) * t13590;
    let t13592 = t2610 * t3614;
    let t13593 = t2365 * t13592;
    let t13594 = t2033 * t13593;
    let t13595 = F::new(0.14896037479937677779e-1) * t13594;
    let t13596 = t11845 * t959;
    (t13585, t13587, t13588, t13589, t13591, t13592, t13593, t13595, t13596)
}
