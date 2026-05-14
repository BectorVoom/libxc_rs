//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 791/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk791<F: Float>(t1691: F, t44: F, t5588: F, t8155: F, t7853: F, t32211: F, t45: F, t1690: F, t1692: F, t1630: F, t383: F, t55: F, t1619: F, t7934: F, t1597: F, t62: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37894 = t1691 * t1691;
    let t37897 = 1.0 / t44 / t8155 / t5588;
    let t37899 = t37894 * t37897 * t7853;
    let t37903 = 1.0 / t45 / t32211;
    let t37905 = t1690 * t1692 * t37903;
    let t37908 = t1630 * t383;
    let t37930 = t8155 * t55;
    let t37931 = 1.0 / t37930;
    let t37935 = t1619 * t7934;
    let t37939 = t1597 * t62;
    (t37894, t37897, t37899, t37903, t37905, t37908, t37931, t37935, t37939)
}
