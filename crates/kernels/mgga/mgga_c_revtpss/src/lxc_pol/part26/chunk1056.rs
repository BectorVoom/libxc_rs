//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1056/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1056<F: Float>(t25949: F, t7063: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F, t3981: F, t2019: F, t3985: F, t820: F, t843: F) -> (F, F, F, F, F, F) {
    let t25950 = t7063 * t25949;
    let t25969 = t7259 * t3974;
    let t25972 = t2482 * t7269 * t27;
    let t25973 = t25972 * t3981;
    let t25975 = t2019 * t3985;
    let t25978 = t820 * t7269 * t843;
    (t25950, t25969, t25972, t25973, t25975, t25978)
}
