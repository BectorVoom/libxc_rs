//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 987/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk987<F: Float>(t19: F, t336: F, t4562: F, t714: F, t16845: F, t16847: F, t16849: F, t16851: F, t16854: F, t16857: F, t16861: F, t16865: F, t16874: F, t16881: F) -> F {
    let t18201 = t4562 * t19 * t336 * t714;
    let t18203 = F::new(0.24311111111111111111e0) * t18201 - t16845 - t16847 - t16849 - t16851 - t16854 + t16857 + t16861 - t16865 - t16874 - t16881;
    t18203
}
