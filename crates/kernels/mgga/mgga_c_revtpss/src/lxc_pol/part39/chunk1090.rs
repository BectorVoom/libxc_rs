//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1090/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1090<F: Float>(t10175: F, t5722: F, t122: F, t5721: F, t3916: F, t9680: F, t1437: F, t1882: F, t2482: F, t4104: F, t10073: F, t5737: F, t1419: F, t4086: F, t543: F, t2782: F) -> (F, F, F, F, F, F) {
    let t14108 = 0.19514881078765566038e-1 * t10175 * t5722;
    let t14109 = t5721 * t122;
    let t14110 = t14109 * t3916;
    let t14111 = t9680 * t14110;
    let t14113 = t1437 * t1882;
    let t14114 = t2482 * t14113;
    let t14116 = 0.19514881078765566038e-1 * t14114 * t4104;
    let t14120 = t10073 * t5737;
    let t14122 = t1419 * t1882;
    let t14124 = t4086 * t14122 * t543;
    let t14126 = 0.10975748638225852664e-1 * t2782 * t14124;
    (t14108, t14111, t14116, t14120, t14122, t14126)
}
