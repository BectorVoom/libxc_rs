//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1171/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1171<F: Float>(t3869: F, t9575: F, t1331: F, t3860: F, t1320: F, t3855: F, t186: F, t685: F, t793: F, t1337: F, t4146: F, t565: F) -> (F, F, F, F, F, F) {
    let t9577 = F::cast_from(0.21687162600603479684e-1_f64) * t3869 * t9575;
    let t9578 = t3860 * t1331;
    let t9580 = t1320 * t3855;
    let t9586 = t685 * t793 * t186;
    let t9588 = F::cast_from(0.56968947174242584612e-3_f64) * t1337 * t9586;
    let t9593 = F::cast_from(1.0_f64) / t4146 / t565;
    (t9577, t9578, t9580, t9586, t9588, t9593)
}
