//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1162/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1162<F: Float>(t2807: F, t8232: F, t10695: F, t311: F, t309: F, t2844: F, t10475: F, t8392: F, t10262: F, t10683: F, t15385: F, t15386: F, t1901: F, t1934: F, t2801: F, t2874: F, t2875: F, t2881: F, t2882: F, t296: F, t4139: F, t4140: F, t42399: F, t42404: F, t4265: F, t43335: F, t446: F, t824: F, t8608: F, t875: F, t882: F) -> (F, F) {
    let t44597 = t8232 * t2807;
    let t44600 = F::cast_from(1.0_f64) / t10695 / t311;
    let t44601 = t309 * t44600;
    let t44602 = t2844 * t2844;
    let t44603 = t44601 * t44602;
    let t44629 = t8392 * t10475;
    let t44634 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t2881 * t2882 * t1934 * t2801 - F::cast_from(8.0_f64) * t446 * t10683 * t882 * t10262 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t44597 + F::cast_from(8.0_f64) * t446 * t296 * t44603 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t1901 * t15385 * t15386 * t42404 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t2874 * t2875 * t8608 * t824 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t2881 * t2882 * t8608 * t875 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t2881 * t4265 * t42399 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1901 * t4139 * t4140 * t42399 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t44629 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t296 * t43335;
    (t44603, t44634)
}
