//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 635/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk635<F: Float>(t8923: F, t8955: F, t8991: F, t9028: F, t82: F, t72: F, t739: F, t9025: F, t2031: F, t4985: F, t2320: F, t7414: F, t1982: F, t2314: F, t7428: F, t2191: F, t2283: F) -> (F, F, F, F, F, F, F, F) {
    let t9030 = t8923 + t8955 + t8991 + t9028;
    let t9031 = t82 * t9030;
    let t9032 = t72 * t9031;
    let t9033 = t739 * t9025;
    let t9035 = t4985 * t2031;
    let t9037 = t7414 * t2320;
    let t9040 = t2314 * t7428 * t1982;
    let t9042 = t2191 * t2283;
    (t9030, t9031, t9032, t9033, t9035, t9037, t9040, t9042)
}
