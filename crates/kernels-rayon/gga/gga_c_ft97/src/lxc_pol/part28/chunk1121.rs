//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1121/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1121(t2179: f64, t5968: f64, t6718: f64, t1882: f64, t35112: f64, t1359: f64, t26768: f64, t35189: f64, t35172: f64, t35176: f64, t35043: f64, t106565: f64, t13140: f64, t1391: f64, t139888: f64, t139896: f64, t144: f64, t167: f64, t1901: f64, t2185: f64, t26523: f64, t26526: f64, t26590: f64, t26909: f64, t27015: f64, t27207: f64, t27211: f64, t27334: f64, t27335: f64, t32729: f64, t33050: f64, t33075: f64, t33125: f64, t3455: f64, t3578: f64, t446: f64, t47659: f64, t574: f64, t5947: f64, t95842: f64) -> (f64, f64, f64) {
    let t147930 = t2179 * t5968 * t6718;
    let t147942 = t1882 * t35112;
    let t147944 = t1359 * t26768;
    let t147949 = t1882 * t35189;
    let t147951 = t1882 * t35172;
    let t147953 = t1882 * t35176;
    let t147978 = t1882 * t35043;
    let t147988 = t446 * t574 * t32729 * t3455 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t144 * t147930 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t1391 * t26909 + 2.0_f64 / 3.0_f64 * t446 * t574 * t3578 * t33125 - t147942 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t167 * t147944 + t147949 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t147951 + t147953 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t574 * t26590 * t5947 + t139888 / 27.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t27015 * t26523 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t27015 * t26526 + t446 * t574 * t3578 * t33050 / 3.0_f64 - t139896 + 4.0_f64 / 9.0_f64 * t47659 * t95842 * t27207 + 4.0_f64 / 9.0_f64 * t47659 * t106565 * t27211 - 2.0_f64 / 9.0_f64 * t147978 - 4.0_f64 * t1901 * t27334 * t27335 * t26523 + t446 * t574 * t3578 * t33075 / 3.0_f64;
    (t147930, t147944, t147988)
}
