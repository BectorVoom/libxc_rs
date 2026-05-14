//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1325/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1325<F: Float>(t18934: F, t18941: F, t18973: F, t18975: F, t18979: F, t18984: F, t18990: F, t18995: F, t19013: F, t23719: F, t23739: F, t23741: F, t23742: F, t23750: F, t23751: F, t23752: F) -> (F,) {
    let t25023 = t18934 - t18941 - t23719 + t18973 + t18975 - t18979 + t18984 - t23739 - t18990 + t23741 - t23742 + t18995 + t23750 + t23751 - t23752 + t19013;
    (t25023,)
}
