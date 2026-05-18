//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1242/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1242<F: Float>(t11281: F, t2011: F, t13281: F, t1617: F, t3659: F, t11224: F, t518: F, t13850: F, t25042: F, t190: F, t467: F, t13853: F) -> (F, F, F, F, F, F) {
    let t35375 = t11281 * t2011;
    let t35378 = F::new(24.0) * t13281 * t3659 * t1617;
    let t35379 = t518 * t11224;
    let t35381 = t25042 * t13850;
    let t35382 = t467 * t190;
    let t35384 = t35381 * t35382 * t13853;
    (t35375, t35378, t35379, t35381, t35382, t35384)
}
