//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1012/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1012<F: Float>(t10444: F, t8392: F, t309: F, t43917: F, t10760: F, t1882: F, t10697: F, t2801: F, t2844: F, t10492: F, t10493: F, t10503: F, t10688: F, t10763: F, t15369: F, t15402: F, t1901: F, t1934: F, t2405: F, t2413: F, t2682: F, t2739: F, t2862: F, t2867: F, t2874: F, t2875: F, t2881: F, t296: F, t4139: F, t42404: F, t44013: F, t446: F, t840: F, t870: F, t871: F) -> (F, F) {
    let t44226 = t8392 * t10444;
    let t44245 = t43917 * t309;
    let t44255 = t1882 * t10760;
    let t44262 = t10697 * t2844 * t2801;
    let t44271 = -8.0 / 9.0 * t44226 - 8.0 * t1901 * t15369 * t870 * t2739 * t2867 - 4.0 / 3.0 * t1901 * t2881 * t10503 * t44013 + 8.0 / 3.0 * t1901 * t4139 * t15402 * t42404 - 4.0 / 3.0 * t1901 * t10492 * t10493 * t2413 - 8.0 / 9.0 * t1901 * t44245 * t10493 * t2405 + 2.0 / 3.0 * t1901 * t2874 * t2875 * t1934 * t2739 + 40.0 / 243.0 * t44255 - 8.0 * t446 * t840 * t10688 * t10763 - 12.0 * t446 * t296 * t44262 - 4.0 * t446 * t2862 * t871 * t2801 * t2682;
    (t44262, t44271)
}
