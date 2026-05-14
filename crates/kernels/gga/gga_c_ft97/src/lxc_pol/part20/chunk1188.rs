//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1188/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1188<F: Float>(t24330: F, t25049: F, t28667: F, t14763: F, t6248: F, t111837: F, t28676: F, t14770: F, t27659: F, t35455: F, t108758: F, t108817: F, t108871: F, t108922: F, t109120: F, t18: F, t25077: F, t2689: F, t28552: F, t28603: F, t28617: F, t54891: F, t6256: F, t6774: F, t6795: F, t6986: F, t704: F, t820: F) -> (F, F) {
    let t112055 = 0.13335600218518518519e0 * t25049 * t24330 * t28667;
    let t112060 = t14763 * t6248;
    let t112071 = t28676 * t111837;
    let t112073 = t27659 * t35455 * t14770;
    let t112078 = -0.13335600218518518519e0 * t25077 * t108817 * t704 * t18 * t820 + t112055 - 0.45306850413028723348e0 * t54891 * t6986 - 0.8890400145679012346e-1 * t6256 * t109120 + 0.40006800655555555556e0 * t112060 * t28617 - 0.10947790369858991997e1 * t2689 * t6795 + 0.24163653553615319118e1 * t2689 * t6774 + 0.40279602951224778277e-1 * t28603 * t108871 + 0.53706137268299704369e-1 * t28603 * t108922 - 0.96671047082939467864e0 * t112071 * t112073 - 0.51860667516460905352e-1 * t28552 * t108758;
    (t112073, t112078)
}
