//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1031/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1031<F: Float>(t12978: F, t2911: F, t8236: F, t133: F, t42680: F, t42661: F, t42304: F, t525: F, t13062: F, t751: F, t12381: F, t532: F) -> (F, F, F, F, F, F) {
    let t42806 = t2911 * t8236 * t12978;
    let t42825 = t133 * t42680;
    let t42827 = t133 * t42661;
    let t42842 = t525 * t42304;
    let t42848 = t751 * t13062;
    let t42876 = t532 * t12381;
    (t42806, t42825, t42827, t42842, t42848, t42876)
}
