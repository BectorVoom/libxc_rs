//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 209/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk209<F: Float>(t257: F, t748: F, t105: F, t107: F, t260: F, t269: F, t438: F, t446: F, t447: F, t751: F) -> (F, F) {
    let t780 = t257 * t748;
    let t786 = 0.33843946638888888889e-3 * t105 * t438 * t269 - 0.25382959979166666667e-3 * t446 * t447 * t269 - 0.50765919958333333334e-3 * t105 * t107 * t780 - 4.0 * t260 * t751;
    (t780, t786)
}
