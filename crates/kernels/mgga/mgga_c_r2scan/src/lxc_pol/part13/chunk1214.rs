//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1214/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1214<F: Float>(t3262: F, t3263: F, t40579: F, t37580: F, t40536: F, t40539: F, t40541: F, t40544: F, t40547: F, t40551: F, t40554: F, t40556: F, t40560: F, t40564: F, t40569: F, t40571: F, t40578: F) -> (F, F) {
    let t40582 = F::new(3.0) / F::new(4.0) * t3262 * t3263 * t40579;
    let t40583 = t40536 + t40539 - t40541 - t40544 - t40547 - t40551 + t40554 + F::new(0.81300399444200075504e-3) * t40556 + t40560 - F::new(0.43368970657079495312e-4) * t40564 - t40569 - t40571 + F::new(0.68400385060046895006e-6) * t37580 + t40578 + t40582;
    (t40582, t40583)
}
