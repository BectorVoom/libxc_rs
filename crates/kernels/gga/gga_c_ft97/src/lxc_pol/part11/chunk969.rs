//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 969/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk969<F: Float>(t10036: F, t1882: F, t9845: F, t10086: F, t8392: F, t2492: F, t2542: F, t10018: F, t10052: F, t10053: F, t10121: F, t13857: F, t1901: F, t193: F, t1934: F, t241: F, t2469: F, t2569: F, t2574: F, t258: F, t2602: F, t2606: F, t265: F, t42252: F, t446: F, t713: F, t729: F, t762: F, t766: F, t89: F, t9692: F) -> (F,) {
    let t42961 = t1882 * t10036;
    let t42978 = t1882 * t9845;
    let t42994 = t8392 * t10086;
    let t42996 = t2492 * t2542;
    let t43005 = 8.0 / 3.0 * t42961 + 8.0 * t446 * t729 * t10052 * t10053 * t713 + 8.0 / 3.0 * t446 * t2574 * t265 * t9692 * t713 + 4.0 / 3.0 * t446 * t729 * t762 * t9692 * t766 - 8.0 / 3.0 * t42978 + 4.0 * t446 * t729 * t2469 * t10018 + 4.0 / 3.0 * t446 * t729 * t762 * t10121 * t713 + t89 * t193 * t241 * t42252 * t258 / 3.0 - 8.0 / 9.0 * t42994 + 4.0 / 3.0 * t1901 * t42996 * t2602 - 4.0 / 3.0 * t1901 * t2606 * t13857 * t1934 * t2569;
    (t43005,)
}
