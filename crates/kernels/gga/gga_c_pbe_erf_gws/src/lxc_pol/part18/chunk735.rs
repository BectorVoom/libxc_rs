//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 735/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk735<F: Float>(t38: F, t531: F, t1477: F, t2060: F, t279: F, t2059: F, t116: F, t784: F, t799: F, t798: F, t259: F, t47: F) -> (F, F, F, F, F) {
    let t4258 = t38 * t531;
    let t4259 = F::new(1.0) / t4258;
    let t4339 = t2060 * t1477 * t279;
    let t4340 = t2059 * t4339;
    let t4341 = F::new(0.31636214830824236053e1) * t4340;
    let t4347 = t799 * t784 * t116;
    let t4348 = t798 * t4347;
    let t4349 = F::new(0.18256146151140740741e1) * t4348;
    let t4351 = F::new(1.0) / t47 / t259;
    (t4258, t4259, t4341, t4349, t4351)
}
