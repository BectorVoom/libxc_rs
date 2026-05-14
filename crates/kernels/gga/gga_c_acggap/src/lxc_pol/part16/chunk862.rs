//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 862/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk862<F: Float>(t33841: F, t7839: F, t8869: F, t1411: F, t1992: F, t7585: F, t7842: F, t31699: F, t8665: F, t30409: F, t30418: F, t31309: F, t525: F, t2016: F, t8622: F, t515: F, t7852: F) -> (F, F, F, F, F, F, F) {
    let t33842 = 0.18868855373762491241e-2 * t33841;
    let t33843 = t7839 * t8869;
    let t33844 = 0.31448092289604152068e-3 * t33843;
    let t33851 = t7585 * t7842 * t1992 * t1411;
    let t33852 = 0.20965394859736101378e-3 * t33851;
    let t33853 = t31699 * t8665;
    let t33857 = t31309 * t30418 * t30409 * t525;
    let t33859 = t2016 * t8622;
    let t33860 = 11.0 / 576.0 * t33859;
    let t33861 = t7852 * t515;
    (t33842, t33844, t33852, t33853, t33857, t33860, t33861)
}
