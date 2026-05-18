//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 850/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk850<F: Float>(t7927: F, t880: F, t7884: F, t7911: F, t7887: F, t7930: F, t862: F, t309: F, t871: F, t620: F, t1210: F, t618: F) -> (F, F, F, F, F) {
    let t29973 = F::new(0.19756347548806534796e1) * t7927 * t880;
    let t29976 = t7884 * t7911;
    let t29977 = t29976 * t7887;
    let t29979 = t862 * t7930;
    let t29980 = t871 * t309;
    let t29982 = t29979 * t620 * t29980;
    let t29984 = t1210 * t618;
    (t29973, t29977, t29979, t29982, t29984)
}
