//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2267/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2267<F: Float>(t25191: F, t7898: F, t1937: F, t49686: F, t75667: F, t13426: F, t6993: F, t101436: F, t101439: F, t101472: F, t101476: F, t101482: F, t101485: F, t101486: F, t101515: F, t101542: F, t13514: F, t1502: F, t1911: F, t2007: F, t2322: F, t25800: F, t25835: F, t27145: F, t27830: F, t28053: F, t3813: F, t569: F, t651: F, t670: F, t7725: F) -> F {
    let t101546 = F::cast_from(6.0_f64) * t7898 * t25191;
    let t101548 = F::cast_from(2.0_f64) * t49686 * t1937;
    let t101550 = F::cast_from(4.0_f64) * t75667 * t1937;
    let t101552 = F::cast_from(4.0_f64) * t13426 * t6993;
    let t101555 = t25835 * t1911 + t101436 + t101439 - F::cast_from(2.0_f64) * t651 * t2007 * t13514 - F::cast_from(4.0_f64) * t2322 * t28053 - F::cast_from(4.0_f64) * t651 * t27830 * t670 - t101472 + t101476 - F::cast_from(4.0_f64) * t2322 * t27145 - t101482 - t101485 - t101486 + (t101515 + t101542) * t569 + t101546 - t101548 - t101550 - t101552 - t1502 * t25800 - t7725 * t3813;
    t101555
}
