//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 979/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk979<F: Float>(t2179: F, t5968: F, t6718: F, t1882: F, t35112: F, t1359: F, t26768: F, t35189: F, t35172: F, t35176: F, t35043: F, t106565: F, t13140: F, t1391: F, t139888: F, t139896: F, t144: F, t167: F, t1901: F, t2185: F, t26523: F, t26526: F, t26590: F, t26909: F, t27015: F, t27207: F, t27211: F, t27334: F, t27335: F, t32729: F, t33050: F, t33075: F, t33125: F, t3455: F, t3578: F, t446: F, t47659: F, t574: F, t5947: F, t95842: F) -> (F, F, F) {
    let t147930 = t2179 * t5968 * t6718;
    let t147942 = t1882 * t35112;
    let t147944 = t1359 * t26768;
    let t147949 = t1882 * t35189;
    let t147951 = t1882 * t35172;
    let t147953 = t1882 * t35176;
    let t147978 = t1882 * t35043;
    let t147988 = t446 * t574 * t32729 * t3455 / 3.0 + 4.0 / 3.0 * t446 * t144 * t147930 + 4.0 / 3.0 * t446 * t2185 * t1391 * t26909 + 2.0 / 3.0 * t446 * t574 * t3578 * t33125 - t147942 / 9.0 + 4.0 / 3.0 * t446 * t2185 * t167 * t147944 + t147949 / 9.0 - 2.0 / 9.0 * t147951 + t147953 / 9.0 + 2.0 / 3.0 * t446 * t574 * t26590 * t5947 + t139888 / 27.0 - 4.0 / 3.0 * t1901 * t13140 * t27015 * t26523 - 4.0 / 3.0 * t1901 * t13140 * t27015 * t26526 + t446 * t574 * t3578 * t33050 / 3.0 - t139896 + 4.0 / 9.0 * t47659 * t95842 * t27207 + 4.0 / 9.0 * t47659 * t106565 * t27211 - 2.0 / 9.0 * t147978 - 4.0 * t1901 * t27334 * t27335 * t26523 + t446 * t574 * t3578 * t33075 / 3.0;
    (t147930, t147944, t147988)
}
