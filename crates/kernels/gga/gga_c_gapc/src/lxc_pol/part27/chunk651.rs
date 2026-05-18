//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 651/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk651<F: Float>(t1615: F, t572: F, t574: F, t177: F, t442: F, t505: F, t1037: F, t1431: F, t1036: F, t1386: F, t515: F, t1552: F, t200: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4908 = t572 * t1615;
    let t4913 = t574 * t574;
    let t4914 = F::new(1.0) / t4913;
    let t4915 = t177 * t4914;
    let t4925 = t442 * t505;
    let t4939 = t1037 * t1431;
    let t4940 = t1036 * t4939;
    let t4948 = t1386 * t515;
    let t4961 = t1552 * t200;
    (t4908, t4913, t4914, t4915, t4925, t4939, t4940, t4948, t4961)
}
