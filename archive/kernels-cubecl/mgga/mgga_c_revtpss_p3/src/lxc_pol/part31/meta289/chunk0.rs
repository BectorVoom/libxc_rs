//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1275/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1275<F: Float>(t1363: F, t9288: F, t1362: F, t3911: F, t3920: F, t2237: F, t240: F, t550: F, t816: F, t1379: F, t2689: F, t3952: F) -> (F, F, F, F, F, F, F) {
    let t9692 = t1363 * t9288;
    let t9694 = F::cast_from(0.30356481678079769392e-1_f64) * t1362 * t9692;
    let t9695 = t3911 * t3920;
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    let t9711 = F::cast_from(0.12846167376791569079e-2_f64) * t1379 * t9709;
    let t9712 = t2689 * t3952;
    (t9692, t9694, t9695, t9707, t9709, t9711, t9712)
}
