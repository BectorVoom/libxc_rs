//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 959/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk959<F: Float>(t11520: F, t11588: F, t300: F, t2979: F, t983: F, t11291: F, t11293: F, t11296: F, t11303: F, t11382: F, t11390: F, t11392: F, t11394: F, t11398: F) -> (F, F, F) {
    let t11590 = t300 * (t11520 + t11588);
    let t11591 = t300 * t2979;
    let t11593 = F::cast_from(0.17544670867903938621e1_f64) * t11591 * t983;
    let t11594 = t11291 + t11293 + t11296 - t11303 + t11382 + t11390 - t11392 - t11394 - t11398 + t11590 - t11593;
    (t11590, t11593, t11594)
}
