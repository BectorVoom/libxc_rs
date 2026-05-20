//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1876/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1876<F: Float>(t7407: F, t93179: F, t25365: F, t26506: F, t25305: F, t95540: F, t10115: F, t2063: F, t213: F, t26473: F, t10982: F, t2061: F, t9646: F) -> (F, F, F, F, F, F) {
    let t95876 = t93179 * t7407;
    let t95888 = t25365 * t26506;
    let t95891 = F::cast_from(0.91399340044406952588e-2_f64) * t25305 * t95540;
    let t95893 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t2063;
    let t95894 = t213 * t26473;
    let t95899 = F::cast_from(0.19637199382202157274e-3_f64) * t9646 * t2061 * t10982;
    (t95876, t95888, t95891, t95893, t95894, t95899)
}
