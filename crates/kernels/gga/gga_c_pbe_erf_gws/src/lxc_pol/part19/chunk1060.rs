//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1060/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1060<F: Float>(t2345: F, t3814: F, t9375: F, t11732: F, t858: F, t867: F, t866: F, t11737: F, t2210: F, t884: F, t2164: F, t3880: F) -> (F, F, F, F) {
    let t11901 = t2345 * t9375 * t3814;
    let t11905 = t867 * t858 * t11732;
    let t11907 = t866 * t11905 / F::new(96.0);
    let t11909 = t2210 * t858 * t11737;
    let t11911 = t884 * t11909 / F::new(16.0);
    let t11912 = t2164 * t3880;
    (t11901, t11907, t11911, t11912)
}
