//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 679/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk679<F: Float>(t2: F, t9895: F, t9771: F, t2493: F, t9775: F, t1775: F, t2499: F, t2494: F, t740: F, t8282: F, t2487: F, t9571: F, t737: F, t3917: F, t9592: F, t9802: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9896 = t9895 * t2;
    let t9897 = t9896 * t9771;
    let t9900 = t2493 * t9775;
    let t9903 = t1775 * t2499;
    let t9905 = t1775 * t2494;
    let t9907 = t8282 * t740;
    let t9909 = t2487 * t9571;
    let t9910 = t737 * t9909;
    let t9913 = t3917 * t9592;
    let t9916 = t9802 * t2;
    (t9896, t9897, t9900, t9903, t9905, t9907, t9909, t9910, t9913, t9916)
}
