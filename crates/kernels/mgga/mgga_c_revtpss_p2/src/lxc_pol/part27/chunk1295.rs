//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1295/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1295<F: Float>(t1459: F, t26120: F, t26124: F, t26127: F, t1455: F, t7700: F, t1464: F, t7690: F, t2167: F, t4168: F, t27089: F, t575: F) -> (F, F, F, F, F, F, F) {
    let t95171 = F::new(18.0) * t1459 * t26120;
    let t95173 = F::new(36.0) * t1459 * t26124;
    let t95175 = F::new(18.0) * t1459 * t26127;
    let t96684 = t1455 * t7700;
    let t96690 = t7690 * t1464;
    let t96692 = t2167 * t4168;
    let t96694 = t27089 * t575;
    (t95171, t95173, t95175, t96684, t96690, t96692, t96694)
}
