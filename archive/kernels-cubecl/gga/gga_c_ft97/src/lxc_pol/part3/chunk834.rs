//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 834/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk834<F: Float>(t4805: F, t609: F, t2179: F, t144: F, t379: F, t4839: F, t569: F, t4824: F, t8392: F, t1017: F, t18: F, t2222: F) -> (F, F, F, F, F) {
    let t16977 = t4805 * t609;
    let t16978 = t2179 * t16977;
    let t16979 = t144 * t16978;
    let t16983 = t569 * t4839 * t379;
    let t16986 = t8392 * t4824;
    let t16988 = t18 * t1017;
    let t16989 = t2222 * t16988;
    (t16978, t16979, t16983, t16986, t16989)
}
