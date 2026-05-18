//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1185/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1185<F: Float>(t25973: F, t2019: F, t3985: F, t7269: F, t820: F, t843: F, t1416: F, t3999: F, t64: F) -> (F, F, F, F, F) {
    let t25974 = F::new(0.2032800112371413129e-3) * t25973;
    let t25975 = t2019 * t3985;
    let t25976 = F::new(0.11337795902333997111e-1) * t25975;
    let t25978 = t820 * t7269 * t843;
    let t25979 = t25978 * t1416;
    let t25980 = F::new(0.16006300097412701803e-1) * t25979;
    let t25981 = t3999 * t64;
    (t25974, t25976, t25978, t25980, t25981)
}
