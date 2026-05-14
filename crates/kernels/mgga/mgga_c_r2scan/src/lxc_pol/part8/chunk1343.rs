//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1343/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1343<F: Float>(t18894: F, t18908: F, t18916: F, t18920: F, t18922: F, t18930: F, t18934: F, t18941: F, t18973: F, t18979: F, t23708: F, t23711: F, t23715: F, t23719: F, t32093: F, t32125: F, t32127: F) -> (F,) {
    let t32957 = t18894 - t23708 + t32093 + t23711 + t18908 + t23715 + t18916 + t18920 - t18922 - t18930 + t32125 - t32127 + t18934 - t18941 + t23719 + t18973 - t18979;
    (t32957,)
}
