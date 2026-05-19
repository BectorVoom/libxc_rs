//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 831/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk831<F: Float>(t7575: F, t9670: F, t1165: F, t1844: F, t604: F, t2068: F, t1856: F, t2001: F, t7520: F, t7540: F, t7546: F, t7550: F, t7558: F, t7602: F, t7612: F, t7632: F, t7639: F, t7641: F, t7672: F, t9292: F, t9309: F, t9661: F, t9664: F, t9667: F) -> (F, F) {
    let t9671 = t7575 * t9670;
    let t9674 = t1165 * t604 * t1844;
    let t9675 = t2068 * t9674;
    let t9677 = t2001 * t1856;
    let t9679 = -t7520 + t7540 + t7546 + t7550 - t7558 - F::new(0.4584375e-1) * t9661 + F::new(0.22921875e-1) * t9664 + F::new(0.1528125e-1) * t9667 + F::cast_from(0.21437009059034868486e-2_f64) * t9671 - t7602 + t7612 + t7632 + t7639 - t7641 - t9292 + F::cast_from(0.15724046144802076034e-3_f64) * t9675 - t9309 + t7672 - F::cast_from(0.34299214494455789578e-2_f64) * t9677;
    (t9674, t9679)
}
