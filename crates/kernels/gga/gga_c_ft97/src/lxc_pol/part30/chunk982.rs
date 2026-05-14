//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 982/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk982<F: Float>(t1701: F, t28629: F, t150786: F, t7607: F, t153047: F, t800: F, t150688: F, t6243: F, t2725: F, t6789: F, t285: F, t6250: F, t28638: F, t33898: F, t35367: F, t150696: F, t33948: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t153094 = t1701 * t28629;
    let t153104 = t7607 * t150786;
    let t153112 = t800 * t153047;
    let t153113 = t150688 * t6243;
    let t153116 = t2725 * t6789;
    let t153117 = t285 * t153116;
    let t153118 = t150688 * t6250;
    let t153121 = t1701 * t28638;
    let t153124 = t35367 * t33898;
    let t153127 = t33948 * t150696;
    (t153094, t153104, t153112, t153113, t153116, t153117, t153118, t153121, t153124, t153127)
}
