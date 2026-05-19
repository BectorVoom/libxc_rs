//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 861/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk861<F: Float>(t25969: F, t2482: F, t27: F, t7269: F, t3981: F, t2019: F, t3985: F, t820: F, t843: F, t1416: F, t3999: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25970 = F::cast_from(0.27104001498285508387e-3_f64) * t25969;
    let t25972 = t2482 * t7269 * t27;
    let t25973 = t25972 * t3981;
    let t25974 = F::cast_from(0.2032800112371413129e-3_f64) * t25973;
    let t25975 = t2019 * t3985;
    let t25976 = F::cast_from(0.11337795902333997111e-1_f64) * t25975;
    let t25978 = t820 * t7269 * t843;
    let t25979 = t25978 * t1416;
    let t25980 = F::cast_from(0.16006300097412701803e-1_f64) * t25979;
    let t25981 = t3999 * t64;
    (t25970, t25972, t25973, t25974, t25975, t25976, t25978, t25979, t25980, t25981)
}
