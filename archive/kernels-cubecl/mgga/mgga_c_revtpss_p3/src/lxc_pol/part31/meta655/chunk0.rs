//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2195/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2195<F: Float>(t1032: F, t6888: F, t1426: F, t7063: F, t7286: F, t1955: F, t30016: F, t686: F, t72: F, t94674: F, t94669: F, t1358: F, t212: F, t30055: F, t689: F) -> (F, F, F, F, F, F) {
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    let t108279 = t7063 * t108278;
    let t108280 = t108279 * t7286;
    let t108282 = t1955 * t108277;
    let t108293 = t30016 * t72 * t686;
    let t108294 = t94674 * t108293;
    let t108296 = t94669 * t108293;
    let t108302 = t689 * t212 * t30055 * t1358;
    (t108278, t108280, t108282, t108294, t108296, t108302)
}
