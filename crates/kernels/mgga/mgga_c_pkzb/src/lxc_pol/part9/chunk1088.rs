//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1088/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1088<F: Float>(t1893: F, t5804: F, t17348: F, t1932: F, t5873: F, t1979: F, t5576: F, t2155: F, t2027: F, t5728: F, t2899: F, t5966: F, t5974: F) -> (F, F, F, F, F, F, F, F) {
    let t17660 = t1893 * t5804;
    let t17664 = F::cast_from(0.17757530864197530864e0_f64) * t17348;
    let t17707 = t1932 * t5873;
    let t17724 = t5576 * t1979;
    let t17728 = F::cast_from(0.18467901234567901234e0_f64) * t17348;
    let t17752 = t2155 * t2155;
    let t17753 = F::cast_from(1.0_f64) / t17752;
    let t17765 = t2027 * t5728;
    let t17780 = t2899 * t5974 * t5966;
    (t17660, t17664, t17707, t17724, t17728, t17753, t17765, t17780)
}
