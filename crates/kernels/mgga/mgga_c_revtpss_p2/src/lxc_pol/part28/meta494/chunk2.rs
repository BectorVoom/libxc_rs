//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1873/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1873<F: Float>(t1398: F, t543: F, t7274: F, t7301: F, t2022: F, t4056: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F) -> (F, F, F, F) {
    let t25960 = t7274 * t1398 * t543;
    let t25961 = t7301 * t25960;
    let t25965 = t2022 * t4056 * t543;
    let t25966 = t7301 * t25965;
    let t25969 = t7259 * t3974;
    let t25970 = F::cast_from(0.27104001498285508387e-3_f64) * t25969;
    let t25972 = t2482 * t7269 * t27;
    (t25961, t25966, t25970, t25972)
}
