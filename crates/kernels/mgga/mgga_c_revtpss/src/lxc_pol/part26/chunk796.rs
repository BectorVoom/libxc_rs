//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 796/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk796<F: Float>(t10227: F, t10228: F, t2349: F, t658: F, t2256: F, t9343: F, t100: F, t106: F, t107: F, t2358: F, t661: F, t2357: F) -> (F, F, F, F, F, F) {
    let t10229 = t10227 * t10228;
    let t10232 = t2349 * t658;
    let t10233 = t10232 * t2256;
    let t10236 = F::new(3.0) * t9343;
    let t10237 = t100 * t10236;
    let t10240 = t107 * t106;
    let t10241 = F::new(1.0) / t10240;
    let t10242 = t2358 * t661;
    let t10243 = t10241 * t10242;
    let t10246 = t2357 * t661;
    (t10229, t10233, t10236, t10237, t10243, t10246)
}
