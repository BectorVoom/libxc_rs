//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 780/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk780<F: Float>(t12472: F, t606: F, t12476: F, t10756: F, t10758: F, t10760: F, t12462: F, t12466: F, t12470: F, t12474: F, t12478: F, t12482: F, t25: F) -> (F, F, F) {
    let t12676 = t606 * t12472;
    let t12679 = t606 * t12476;
    let t12682 = -F::new(0.39990740740740740742e-1) * t12462 - F::new(0.35991666666666666667e-1) * t12482 + F::new(0.13333333333333333334e-1) * t10756 + F::new(0.44444444444444444445e-2) * t10758 - F::new(0.26666666666666666667e-1) * t10760 + F::new(0.14396666666666666667e0) * t12466 - F::new(0.71983333333333333335e-1) * t12470 - F::new(0.21595e0) * t12474 + F::new(0.21595e0) * t12478 - F::new(0.39999999999999999999e-1) * t25 * t12676 + F::new(0.39999999999999999999e-1) * t25 * t12679;
    (t12676, t12679, t12682)
}
