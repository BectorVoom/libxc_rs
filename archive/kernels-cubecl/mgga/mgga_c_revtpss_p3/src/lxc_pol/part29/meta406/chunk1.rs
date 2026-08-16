//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1472/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1472<F: Float>(t11509: F, t2988: F, t15541: F, t981: F, t15100: F, t15103: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15399: F, t15519: F, t15522: F, t15524: F, t15528: F, t15530: F, t15536: F, t15540: F, t3329: F, t5023: F, t5024: F) -> (F, F) {
    let t15542 = t11509 * t2988;
    let t15543 = t15541 * t15542;
    let t15545 = F::cast_from(0.10254018858216406658e4_f64) * t981 * t15543;
    let t15546 = -t3329 * t5023 * t5024 + t15100 - t15103 - t15377 + t15379 - t15382 - t15385 - t15388 + t15392 + t15395 + t15399 + t15519 + t15522 - t15524 - t15528 + t15530 - t15536 + t15540 - t15545;
    (t15545, t15546)
}
