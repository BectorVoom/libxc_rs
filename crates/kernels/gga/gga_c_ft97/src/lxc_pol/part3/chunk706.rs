//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 706/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk706<F: Float>(t15756: F, t3134: F, t15742: F, t3127: F, t11690: F, t15737: F, t15746: F, t15932: F, t1787: F, t15940: F, t8327: F, t1587: F, t3103: F, t3149: F, t1775: F, t4519: F) -> (F, F, F, F, F, F, F, F) {
    let t16418 = t3134 * t15756;
    let t16421 = t3127 * t15742;
    let t16424 = t11690 * t15737;
    let t16427 = t3127 * t15746;
    let t16430 = t1787 * t15932;
    let t16433 = t8327 * t15940;
    let t16439 = t1587 * t3149 * t3103;
    let t16442 = t1775 * t4519;
    (t16418, t16421, t16424, t16427, t16430, t16433, t16439, t16442)
}
