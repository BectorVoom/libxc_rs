//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1020/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1020<F: Float>(t38953: F, t6163: F, t24794: F, t8392: F, t1882: F, t24804: F, t6081: F, t8232: F, t6090: F, t24765: F, t24693: F, t24770: F, t24711: F, t24614: F, t24761: F, t24583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t97952 = t38953 * t6163;
    let t97957 = t8392 * t24794;
    let t97962 = t1882 * t24804;
    let t97964 = t8232 * t6081;
    let t97966 = t8232 * t6090;
    let t97978 = t1882 * t24765;
    let t98001 = t1882 * t24693;
    let t98016 = t1882 * t24770;
    let t98021 = t1882 * t24711;
    let t98029 = t1882 * t24614;
    let t98051 = t8392 * t24761;
    let t98053 = t1882 * t24583;
    (t97952, t97957, t97962, t97964, t97966, t97978, t98001, t98016, t98021, t98029, t98051, t98053)
}
