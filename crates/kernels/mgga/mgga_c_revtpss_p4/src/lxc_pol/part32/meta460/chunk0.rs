//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1679/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1679<F: Float>(t1426: F, t545: F, t2453: F, t7283: F, t25920: F, t7063: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F) -> (F, F, F, F, F, F) {
    let t25937 = t1426 * t545;
    let t25944 = t2453 * t7283;
    let t25949 = t25920 * t1426;
    let t25950 = t7063 * t25949;
    let t25969 = t7259 * t3974;
    let t25972 = t2482 * t7269 * t27;
    (t25937, t25944, t25949, t25950, t25969, t25972)
}
