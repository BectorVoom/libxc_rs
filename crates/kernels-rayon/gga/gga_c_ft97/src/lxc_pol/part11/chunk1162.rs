//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1162/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1162(t2807: f64, t8232: f64, t10695: f64, t311: f64, t309: f64, t2844: f64, t10475: f64, t8392: f64, t10262: f64, t10683: f64, t15385: f64, t15386: f64, t1901: f64, t1934: f64, t2801: f64, t2874: f64, t2875: f64, t2881: f64, t2882: f64, t296: f64, t4139: f64, t4140: f64, t42399: f64, t42404: f64, t4265: f64, t43335: f64, t446: f64, t824: f64, t8608: f64, t875: f64, t882: f64) -> (f64, f64) {
    let t44597 = t8232 * t2807;
    let t44600 = 1.0_f64 / t10695 / t311;
    let t44601 = t309 * t44600;
    let t44602 = t2844 * t2844;
    let t44603 = t44601 * t44602;
    let t44629 = t8392 * t10475;
    let t44634 = 2.0_f64 / 3.0_f64 * t1901 * t2881 * t2882 * t1934 * t2801 - 8.0_f64 * t446 * t10683 * t882 * t10262 - 16.0_f64 / 9.0_f64 * t44597 + 8.0_f64 * t446 * t296 * t44603 - 20.0_f64 / 27.0_f64 * t1901 * t15385 * t15386 * t42404 + 4.0_f64 / 9.0_f64 * t1901 * t2874 * t2875 * t8608 * t824 + 4.0_f64 / 9.0_f64 * t1901 * t2881 * t2882 * t8608 * t875 + 8.0_f64 / 9.0_f64 * t1901 * t2881 * t4265 * t42399 - 8.0_f64 / 27.0_f64 * t1901 * t4139 * t4140 * t42399 + 8.0_f64 / 27.0_f64 * t44629 + 8.0_f64 / 3.0_f64 * t446 * t296 * t43335;
    (t44603, t44634)
}
