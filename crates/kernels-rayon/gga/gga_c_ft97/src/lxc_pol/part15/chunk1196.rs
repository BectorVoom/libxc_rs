//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1196/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1196(t10485: f64, t10683: f64, t10703: f64, t1091: f64, t1255: f64, t1901: f64, t21978: f64, t22208: f64, t22212: f64, t2881: f64, t44204: f64, t44518: f64, t446: f64, t4965: f64, t5330: f64, t5414: f64, t72391: f64, t84283: f64, t84312: f64, t84317: f64, t84581: f64, t90603: f64, t90717: f64) -> f64 {
    let t90765 = -4.0_f64 / 9.0_f64 * t84283 + 4.0_f64 / 3.0_f64 * t1901 * t72391 * t5414 + 4.0_f64 / 9.0_f64 * t1901 * t2881 * t84581 * t1091 - 8.0_f64 * t446 * t10683 * t1255 * t21978 + 8.0_f64 / 3.0_f64 * t1901 * t2881 * t44204 * t90717 + 8.0_f64 / 3.0_f64 * t1901 * t2881 * t10485 * t90603 - 4.0_f64 / 9.0_f64 * t84312 - 8.0_f64 / 3.0_f64 * t84317 - 8.0_f64 / 9.0_f64 * t1901 * t44518 * t5330 * t4965 - 4.0_f64 / 3.0_f64 * t1901 * t10703 * t22208 * t1091 - 4.0_f64 / 3.0_f64 * t1901 * t10703 * t22212 * t1091;
    t90765
}
