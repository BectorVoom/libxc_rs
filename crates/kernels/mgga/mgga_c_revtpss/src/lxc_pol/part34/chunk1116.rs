//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1116/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1116<F: Float>(t25953: F, t7289: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F, t2019: F, t3985: F, t820: F, t843: F, t3999: F, t64: F) -> (F, F, F, F, F, F) {
    let t25955 = F::new(0.17135234354032049604e-1) * t7289 * t25953;
    let t25969 = t7259 * t3974;
    let t25970 = F::new(0.27104001498285508387e-3) * t25969;
    let t25972 = t2482 * t7269 * t27;
    let t25975 = t2019 * t3985;
    let t25976 = F::new(0.11337795902333997111e-1) * t25975;
    let t25978 = t820 * t7269 * t843;
    let t25981 = t3999 * t64;
    (t25955, t25970, t25972, t25976, t25978, t25981)
}
