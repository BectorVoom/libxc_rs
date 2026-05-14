//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1273/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1273<F: Float>(t1020: F, t1535: F, t1634: F, t16886: F, t16889: F, t16893: F, t19770: F, t24637: F, t24638: F, t24640: F, t24641: F, t24643: F, t24644: F, t24645: F, t2718: F, t9121: F) -> (F,) {
    let t25026 = 6.0 * t1020 * t1535 * t19770 - 6.0 * t1634 * t2718 * t9121 - t16886 - t16889 + t16893 - t24637 - t24638 + t24640 - t24641 + t24643 - t24644 + t24645;
    (t25026,)
}
