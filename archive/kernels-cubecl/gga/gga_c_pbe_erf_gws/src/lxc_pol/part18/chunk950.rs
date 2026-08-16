//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 950/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk950<F: Float>(t10702: F, t617: F, t1621: F, t1620: F, t10668: F, t10670: F, t10674: F, t10678: F, t10683: F, t10687: F, t10690: F, t10695: F, t10697: F, t10699: F, t10701: F, t7526: F, t7532: F, t7541: F, t7572: F, t7573: F) -> (F, F) {
    let t10703 = t10702 * t617;
    let t10704 = t1621 * t10703;
    let t10706 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1620 * t10704;
    let t10707 = t10668 - t10670 + t7526 - t7532 + t10674 - t10678 + t10683 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t7541 - t10687 - t7572 + F::cast_from(0.66490888888888888886e-1_f64) * t7573 - t10690 - t10695 - t10697 + t10699 + t10701 - t10706;
    (t10706, t10707)
}
