//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 816/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk816<F: Float>(t16832: F, t16834: F, t16838: F, t16842: F, t16845: F, t16847: F, t16849: F, t16851: F, t16854: F, t16857: F, t16861: F, t16865: F, t1413: F, t1642: F, t1724: F, t5522: F, t639: F) -> (F, F) {
    let t16866 = t16832 - t16834 + t16838 - t16842 - t16845 - t16847 - t16849 - t16851 - t16854 + t16857 + t16861 - t16865;
    let t16874 = 8.0 / 9.0 * t639 * t5522 * t1724 * t1642 * t1413;
    (t16866, t16874)
}
