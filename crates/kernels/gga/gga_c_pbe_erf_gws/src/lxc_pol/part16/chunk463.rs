//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 463/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk463<F: Float>(t1759: F, t1856: F, t1765: F, t606: F, t1769: F, t1756: F, t1761: F, t1767: F, t1771: F, t1844: F, t1851: F, t1852: F, t25: F) -> (F, F, F, F) {
    let t1857 = t1856 * t1759;
    let t1860 = t606 * t1765;
    let t1863 = t606 * t1769;
    let t1866 = t1844 + F::new(0.23994444444444444444e-1) * t1756 - F::new(0.23994444444444444445e-1) * t1761 + F::new(0.71983333333333333334e-1) * t1767 - F::new(0.35991666666666666667e-1) * t1771 + t1851 + F::new(0.8888888888888888889e-2) * t1852 - F::new(0.22222222222222222222e-2) * t25 * t1857 + F::new(0.13333333333333333333e-1) * t25 * t1860 - F::new(0.66666666666666666667e-2) * t25 * t1863;
    (t1857, t1860, t1863, t1866)
}
