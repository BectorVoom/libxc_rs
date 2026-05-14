//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1214/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1214<F: Float>(t24324: F, t24330: F, t30590: F, t108517: F, t108550: F, t108576: F, t108583: F, t108738: F, t108806: F, t109033: F, t1091: F, t109245: F, t109247: F, t109273: F, t1096: F, t122787: F, t122797: F, t122800: F, t122803: F, t122820: F, t122824: F, t122827: F, t122830: F, t122836: F, t122840: F, t122841: F, t122846: F, t122849: F, t14832: F, t2248: F, t231: F, t232: F, t24265: F, t24276: F, t24277: F, t24278: F, t27500: F, t27522: F, t27659: F, t35409: F, t3746: F, t3751: F, t66565: F, t96599: F, t96615: F, t96696: F) -> (F,) {
    let t122852 = t24324 * t24330 * t30590;
    let t122855 = -0.89080607335887169332e-3 * t24265 * t232 * t122787 + 0.20715606998445758511e-4 * t108738 * t96615 * t231 * t66565 + 0.17659850543899795697e-2 * t122797 - 0.6809984893827160494e-1 * t122800 - 0.14846767889314528222e-3 * t24276 * t109033 * t122803 + 0.14846767889314528222e-3 * t24276 * t24278 * t3751 * t1091 - 0.34526011664076264185e-5 * t109245 * t109247 * t14832 * t1091 - 0.29693535778629056444e-3 * t24276 * t2248 * t24277 * t1096 * t3746 + t108550 + 0.10357803499222879255e-4 * t122820 * t96696 + 0.7423383944657264111e-4 * t122824 - 0.51074886703703703704e-1 * t27500 * t122827 + 0.36328835404593865432e-2 * t122830 * t27659 * t35409 * t27522 + t108576 - 0.59346127734643676855e-4 * t108517 * t96599 * t109273 * t122836 + 0.267241822007661508e-2 * t108806 * t122840 * t122841 + 0.6809984893827160494e-1 * t122846 + 0.12768721675925925926e-1 * t122849 - 0.38306165027777777777e-1 * t122852 - 0.3959138103817207526e-3 * t108583;
    (t122855,)
}
