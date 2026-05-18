//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 463/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk463<F: Float>(t2036: F, t525: F, t169: F, t301: F, t745: F, t784: F, t125: F, t143: F, t1452: F, t1499: F, t1501: F, t1503: F, t1504: F, t1593: F, t1944: F, t2024: F, t2026: F, t2033: F, t2035: F, t279: F, t299: F, t475: F, t523: F, t526: F) -> (F, F) {
    let t2037 = t525 * t2036;
    let t2042 = t169 * t784 * t745 * t301;
    let t2048 = -t1499 + t523 * t1501 + F::new(6.0) * t1503 * t143 * t1504 + t1593 * t526 + t1944 * t279 + t2024 * t125 + F::new(3.0) * t475 * t2026 - t523 * t2033 + F::new(6.0) * t2035 * t2037 - F::new(0.10809180959278284142e0) * t2042 + F::new(0.20267214298646782767e-1) * t169 * t299 * t1452 * t301;
    (t2037, t2048)
}
