//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1417/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1417<F: Float>(t141: F, t2908: F, t41263: F, t11321: F, t698: F, t2297: F, t2851: F, t39443: F, t11341: F, t11331: F, t11144: F, t2439: F, t2912: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41265 = t141 * t2908 * t41263;
    let t41267 = t698 * t11321;
    let t41270 = F::new(1.0) / t2851 / t2297;
    let t41271 = t41270 * t39443;
    let t41273 = t141 * t11341 * t41271;
    let t41275 = t698 * t11331;
    let t41277 = t11144 * t39443;
    let t41279 = t141 * t2908 * t41277;
    let t41281 = t2439 * t2912;
    (t41265, t41267, t41270, t41271, t41273, t41275, t41277, t41279, t41281)
}
