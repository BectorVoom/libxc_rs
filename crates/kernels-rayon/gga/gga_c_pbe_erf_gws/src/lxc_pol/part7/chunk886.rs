//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 886/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk886(t16832: f64, t16834: f64, t16838: f64, t16842: f64, t16845: f64, t16847: f64, t16849: f64, t16851: f64, t16854: f64, t16857: f64, t16861: f64, t16865: f64) -> f64 {
    let t16866 = t16832 - t16834 + t16838 - t16842 - t16845 - t16847 - t16849 - t16851 - t16854 + t16857 + t16861 - t16865;
    t16866
}
