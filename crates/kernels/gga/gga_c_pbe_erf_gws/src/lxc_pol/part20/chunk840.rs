//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 840/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk840<F: Float>(t8122: F, t8124: F, t485: F, t974: F, t156: F, t2881: F, t496: F, t2874: F, t395: F, t1508: F, t971: F, t1251: F) -> (F, F, F, F, F) {
    let t8126 = F::new(0.587616e1) * t8122 * t8124;
    let t8135 = t485 * t974;
    let t8137 = F::cast_from(0.19486833333333333333e1_f64) * t8135 * t8124;
    let t8139 = t496 * t156 * t2881;
    let t8140 = t485 * t2874;
    let t8142 = F::cast_from(0.97434166666666666666e0_f64) * t8140 * t395;
    let t8143 = t1508 * t971;
    let t8144 = t8143 * t1251;
    (t8126, t8137, t8139, t8142, t8144)
}
