//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 829/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk829<F: Float>(t185: F, t7751: F, t2730: F, t2753: F, t1639: F, t649: F, t1642: F, t1730: F, t1: F, t837: F, t2736: F, t616: F) -> (F, F, F, F, F, F) {
    let t7753 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t185 * t7751;
    let t7757 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2730 * t2753;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7775 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1730 * t2753;
    let t7776 = t1 * t837;
    let t7777 = t7776 * t2736;
    let t7778 = t616 * t7777;
    (t7753, t7757, t7759, t7775, t7776, t7778)
}
