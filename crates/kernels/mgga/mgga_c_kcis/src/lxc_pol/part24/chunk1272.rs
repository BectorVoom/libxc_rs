//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1272/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1272<F: Float>(t100784: F, t7788: F, t19317: F, t303: F, t356: F, t27924: F, t5019: F, t1020: F, t6620: F, t92701: F, t26671: F, t6625: F) -> (F, F, F, F, F) {
    let t100805 = t7788 * t100784;
    let t100814 = t303 * t356 * t19317;
    let t100817 = t303 * t27924 * t5019;
    let t100820 = t1020 * t92701 * t6620;
    let t100823 = t1020 * t26671 * t6625;
    (t100805, t100814, t100817, t100820, t100823)
}
