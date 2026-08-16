//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1152/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1152(t10444: f64, t8392: f64, t309: f64, t43917: f64, t10760: f64, t1882: f64, t10697: f64, t2801: f64, t2844: f64, t10492: f64, t10493: f64, t10503: f64, t10688: f64, t10763: f64, t15369: f64, t15402: f64, t1901: f64, t1934: f64, t2405: f64, t2413: f64, t2682: f64, t2739: f64, t2862: f64, t2867: f64, t2874: f64, t2875: f64, t2881: f64, t296: f64, t4139: f64, t42404: f64, t44013: f64, t446: f64, t840: f64, t870: f64, t871: f64) -> (f64, f64) {
    let t44226 = t8392 * t10444;
    let t44245 = t43917 * t309;
    let t44255 = t1882 * t10760;
    let t44262 = t10697 * t2844 * t2801;
    let t44271 = -8.0_f64 / 9.0_f64 * t44226 - 8.0_f64 * t1901 * t15369 * t870 * t2739 * t2867 - 4.0_f64 / 3.0_f64 * t1901 * t2881 * t10503 * t44013 + 8.0_f64 / 3.0_f64 * t1901 * t4139 * t15402 * t42404 - 4.0_f64 / 3.0_f64 * t1901 * t10492 * t10493 * t2413 - 8.0_f64 / 9.0_f64 * t1901 * t44245 * t10493 * t2405 + 2.0_f64 / 3.0_f64 * t1901 * t2874 * t2875 * t1934 * t2739 + 40.0_f64 / 243.0_f64 * t44255 - 8.0_f64 * t446 * t840 * t10688 * t10763 - 12.0_f64 * t446 * t296 * t44262 - 4.0_f64 * t446 * t2862 * t871 * t2801 * t2682;
    (t44262, t44271)
}
