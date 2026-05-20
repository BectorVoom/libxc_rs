//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1289/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1289<F: Float>(t2035: F, t95019: F, t28167: F, t8996: F, t9984: F, t26090: F, t7235: F, t25082: F, t49640: F, t8717: F, t25191: F, t2322: F, t25861: F) -> (F, F, F, F, F, F) {
    let t95020 = t95019 * t2035;
    let t95023 = F::new(18.0) * t28167 * t8996 * t9984;
    let t95025 = F::new(3.0) * t7235 * t26090;
    let t95032 = F::new(9.0) * t25082 * t8717 * t49640;
    let t95036 = F::new(18.0) * t7235 * t25191;
    let t95038 = F::new(12.0) * t2322 * t25861;
    (t95020, t95023, t95025, t95032, t95036, t95038)
}
