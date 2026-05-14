//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 778/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk778<F: Float>(t8022: F, t96: F, t1674: F, t7278: F, t922: F, t1679: F, t811: F, t9097: F, t7927: F, t880: F, t7884: F, t7911: F, t7887: F, t7930: F, t862: F, t309: F, t871: F) -> (F, F, F, F, F, F, F) {
    let t29955 = t96 * t8022;
    let t29958 = t1674 * t7278 * t922;
    let t29961 = t1679 * t9097 * t811;
    let t29973 = 0.19756347548806534796e1 * t7927 * t880;
    let t29976 = t7884 * t7911;
    let t29977 = t29976 * t7887;
    let t29979 = t862 * t7930;
    let t29980 = t871 * t309;
    (t29955, t29958, t29961, t29973, t29977, t29979, t29980)
}
