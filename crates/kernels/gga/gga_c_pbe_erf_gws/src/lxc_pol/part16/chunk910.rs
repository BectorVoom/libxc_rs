//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 910/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk910<F: Float>(t1885: F, t7936: F, t587: F, t589: F, t837: F, t2621: F, t5557: F, t1023: F, t1672: F, t616: F, t2786: F, t579: F) -> (F, F, F, F, F) {
    let t7937 = t1885 * t7936;
    let t7939 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t587 * t7937;
    let t7940 = t837 * t589;
    let t7941 = t7940 * t2621;
    let t7942 = t587 * t7941;
    let t7943 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t7942;
    let t7944 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t5557;
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7947 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t7946;
    let t7949 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t579 * t2786;
    (t7939, t7943, t7944, t7947, t7949)
}
