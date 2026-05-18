//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1187/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1187<F: Float>(t26502: F, t786: F, t789: F, t93314: F, t95854: F, t7407: F, t93179: F, t25365: F, t26506: F, t25305: F, t95540: F, t10115: F, t2063: F) -> (F, F, F, F, F, F) {
    let t95866 = t786 * t26502 * t789;
    let t95872 = t93314 * t95854;
    let t95876 = t93179 * t7407;
    let t95888 = t25365 * t26506;
    let t95891 = F::new(0.91399340044406952588e-2) * t25305 * t95540;
    let t95893 = F::new(0.11044544084478153697e-3) * t10115 * t2063;
    (t95866, t95872, t95876, t95888, t95891, t95893)
}
