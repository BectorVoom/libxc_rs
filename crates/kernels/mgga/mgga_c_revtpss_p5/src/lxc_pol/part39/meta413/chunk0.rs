//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1490/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1490<F: Float>(t2349: F, t43: F, t10227: F, t96: F, t100: F, t613: F, t10199: F, t2175: F, t2289: F, t8264: F, t31051: F, t625: F) -> (F, F, F, F, F, F) {
    let t116942 = t43 * t2349;
    let t116946 = t96 * t10227;
    let t116957 = t613 * t100;
    let t116968 = F::new(154.0) / F::new(27.0) * t10199 * t2175;
    let t116969 = t2289 * t8264;
    let t116971 = t625 * t31051;
    (t116942, t116946, t116957, t116968, t116969, t116971)
}
