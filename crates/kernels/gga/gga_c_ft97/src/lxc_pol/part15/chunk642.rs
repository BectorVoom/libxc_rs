//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 642/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk642<F: Float>(t375: F, t4669: F, t89: F, t160: F, t4714: F, t1882: F, t4726: F, t4824: F, t8392: F, t2178: F, t4724: F, t4668: F) -> (F, F, F, F, F, F) {
    let t16928 = t89 * t375 * t4669;
    let t16963 = t160 * t4714;
    let t16969 = t1882 * t4726;
    let t16986 = t8392 * t4824;
    let t17016 = t2178 * t4724;
    let t17021 = t160 * t4668;
    (t16928, t16963, t16969, t16986, t17016, t17021)
}
