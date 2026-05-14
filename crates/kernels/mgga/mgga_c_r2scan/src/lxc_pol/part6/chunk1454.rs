//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1454/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1454<F: Float>(t1039: F, t18908: F, t18916: F, t18920: F, t18922: F, t18930: F, t18934: F, t18941: F, t2337: F, t23715: F, t23718: F, t2449: F, t6906: F, t18973: F, t18975: F, t18979: F, t18984: F, t18990: F, t23719: F, t23724: F, t23730: F, t23735: F, t23738: F, t23739: F) -> (F, F) {
    let t27428 = t1039 * t6906 + 3.0 * t2337 * t2449 + t18908 + t18916 + t18920 - t18922 - t18930 + t18934 - t18941 - t23715 - t23718;
    let t27431 = -t23719 + t18973 - t23724 + t18975 - t18979 - t23730 - t23735 - t23738 + t18984 - t23739 - t18990;
    (t27428, t27431)
}
