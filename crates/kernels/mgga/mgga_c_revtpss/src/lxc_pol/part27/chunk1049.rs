//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1049/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1049<F: Float>(t12273: F, t1264: F, t247: F, t1284: F, t3555: F, t3624: F, t12803: F, t3629: F, t3626: F, t1121: F, t3603: F, t606: F) -> (F, F, F, F, F) {
    let t12828 = t247 * t1264 * t12273;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12835 = t12803 * t3629;
    let t12836 = t3626 * t12835;
    let t12839 = t3603 * t1121;
    let t12840 = t12839 * t606;
    (t12828, t12831, t12832, t12836, t12840)
}
