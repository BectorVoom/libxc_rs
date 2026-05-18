//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 544/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk544<F: Float>(t2768: F, t606: F, t1756: F, t1844: F, t1851: F, t1852: F, t25: F, t2718: F, t2760: F, t2763: F, t2766: F, t2770: F, t2773: F, t2775: F, t2778: F) -> (F, F) {
    let t2781 = t606 * t2768;
    let t2784 = t1844 + F::new(0.11997222222222222222e-1) * t1756 + F::new(0.11997222222222222222e-1) * t2760 - F::new(0.23994444444444444445e-1) * t2763 + F::new(0.71983333333333333334e-1) * t2766 - F::new(0.71983333333333333334e-1) * t2770 + t1851 + F::new(0.44444444444444444445e-2) * t1852 + F::new(0.44444444444444444445e-2) * t2773 - F::new(0.22222222222222222222e-2) * t25 * t2775 + F::new(0.13333333333333333333e-1) * t25 * t2778 - F::new(0.13333333333333333333e-1) * t2718 * t2781;
    (t2781, t2784)
}
