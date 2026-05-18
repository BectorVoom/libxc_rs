//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 834/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk834<F: Float>(t2621: F, t7940: F, t587: F, t1023: F, t1672: F, t616: F, t202: F, t2814: F, t184: F, t996: F, t561: F, t2799: F, t7776: F) -> (F, F, F, F, F) {
    let t7941 = t7940 * t2621;
    let t7942 = t587 * t7941;
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7950 = t202 * t2814;
    let t7951 = t7950 * t184;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    let t7959 = t7776 * t2799;
    (t7942, t7946, t7951, t7957, t7959)
}
