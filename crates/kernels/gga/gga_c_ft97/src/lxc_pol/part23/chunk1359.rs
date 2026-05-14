//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1359/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1359<F: Float>(t5284: F, t703: F, t108448: F, t111831: F, t28566: F, t28547: F, t27659: F, t28661: F, t35877: F, t108685: F, t6242: F, t6999: F, t24330: F, t25112: F, t31389: F, t6241: F, t70671: F) -> (F, F, F, F, F, F, F) {
    let t127128 = t703 * t5284;
    let t127135 = t108448 * t111831 * t28566;
    let t127139 = t108448 * t111831 * t28547;
    let t127147 = t27659 * t35877 * t28661;
    let t127151 = t6242 * t108685 * t6999;
    let t127158 = t25112 * t24330 * t31389;
    let t127160 = t70671 * t6241;
    (t127128, t127135, t127139, t127147, t127151, t127158, t127160)
}
