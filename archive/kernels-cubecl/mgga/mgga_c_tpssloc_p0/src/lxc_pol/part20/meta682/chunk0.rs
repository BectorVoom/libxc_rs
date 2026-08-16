//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2574/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2574<F: Float>(t14165: F, t44505: F, t11557: F, t4889: F, t11560: F, t1174: F, t1716: F, t2402: F, t4930: F, t698: F, t11513: F, t11589: F, t15313: F, t3447: F) -> (F, F, F, F, F, F, F) {
    let t52066 = t44505 * t14165;
    let t52074 = t4889 * t11557;
    let t52076 = t4889 * t11560;
    let t52081 = t1174 * t2402 * t1716;
    let t52084 = t1174 * t698 * t4930;
    let t52085 = F::cast_from(0.55555555555555555554e-3_f64) * t52084;
    let t52086 = t4889 * t11513;
    let t52089 = t3447 * t11589 * t15313;
    (t52066, t52074, t52076, t52081, t52085, t52086, t52089)
}
