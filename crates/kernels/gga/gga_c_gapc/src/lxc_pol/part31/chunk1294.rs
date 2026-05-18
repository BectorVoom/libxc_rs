//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1294/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1294<F: Float>(t190: F, t467: F, t13853: F, t35381: F, t3640: F, t8341: F, t11224: F, t2933: F, t1484: F, t11203: F, t8286: F, t8297: F) -> (F, F, F, F, F, F) {
    let t35382 = t467 * t190;
    let t35384 = t35381 * t35382 * t13853;
    let t35386 = t8341 * t3640;
    let t35388 = t2933 * t11224;
    let t35390 = t1484 * t3640;
    let t35393 = t8286 * t11203 * t8297;
    (t35382, t35384, t35386, t35388, t35390, t35393)
}
