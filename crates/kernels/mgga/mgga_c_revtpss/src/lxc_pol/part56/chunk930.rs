//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 930/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk930<F: Float>(t32183: F, t32186: F, t786: F, t7063: F, t1385: F, t239: F) -> (F, F, F) {
    let t32187 = t786 * t32183 * t32186;
    let t32188 = F::cast_from(0.18822977838986977999e-4_f64) * t32187;
    let t32190 = t7063 * t32183 * t32186;
    let t32191 = F::cast_from(0.33467254597718846885e-4_f64) * t32190;
    let t32192 = t1385 * t239;
    (t32188, t32191, t32192)
}
