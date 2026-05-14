//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 452/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk452<F: Float>(t573: F, t610: F, t1827: F, t587: F, t108: F, t1403: F, t1407: F, t1413: F, t1416: F, t726: F, t728: F, t92: F, t93: F, t1754: F, t1706: F, t187: F, t190: F) -> (F, F, F, F, F, F) {
    let t1828 = t573 * t610;
    let t1829 = t1827 * t1828;
    let t1831 = 8.0 / 45.0 * t587 * t1829;
    let t1841 = (20.0 / 9.0 * t92 * t1403 + 4.0 / 3.0 * t726 * t1407 + 20.0 / 9.0 * t93 * t1413 + 4.0 / 3.0 * t728 * t1416) * t108;
    let t1844 = 0.47988888888888888889e-1 * t1754;
    let t1851 = 0.11111111111111111111e-1 * t190 * t1706 * t187;
    (t1828, t1829, t1831, t1841, t1844, t1851)
}
