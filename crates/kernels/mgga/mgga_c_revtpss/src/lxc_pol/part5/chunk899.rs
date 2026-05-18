//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 899/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk899<F: Float>(t1188: F, t6518: F, t3503: F, t3510: F, t5044: F, t5093: F, t6423: F, t6427: F, t6431: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F) -> (F, F) {
    let t6519 = t6518 * t1188;
    let t6534 = -F::new(0.1294625e1) * t6443 + F::new(0.258925e1) * t6450 + t3503 - F::new(0.20128333333333333334e0) * t5044 - F::new(0.20128333333333333333e0) * t6423 + F::new(0.60385e0) * t6427 + F::new(0.301925e0) * t6431 + F::new(0.82524375e-1) * t6456 + F::new(0.16504875e0) * t6458 + t3510 - F::new(0.11038e0) * t5093 - F::new(0.27595e-1) * t6462 + F::new(0.16557e0) * t6465 + F::new(0.82785e-1) * t6468;
    (t6519, t6534)
}
