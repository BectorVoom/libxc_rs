//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 643/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk643<F: Float>(t144: F, t26599: F, t1882: F, t6645: F, t6653: F, t23548: F, t3424: F, t9144: F, t3429: F, t13220: F, t1384: F, t1570: F, t3188: F, t12709: F, t1557: F, t12714: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27199 = t144 * t26599;
    let t27203 = t1882 * t6645;
    let t27205 = t1882 * t6653;
    let t27207 = t23548 * t3424;
    let t27208 = t9144 * t27207;
    let t27211 = t23548 * t3429;
    let t27212 = t13220 * t27211;
    let t27215 = t1384 * t1570;
    let t27216 = t27215 * t3188;
    let t27217 = t12709 * t27216;
    let t27220 = t1384 * t1557;
    let t27221 = t27220 * t3188;
    let t27222 = t12714 * t27221;
    (t27199, t27203, t27205, t27207, t27208, t27211, t27212, t27216, t27217, t27221, t27222)
}
