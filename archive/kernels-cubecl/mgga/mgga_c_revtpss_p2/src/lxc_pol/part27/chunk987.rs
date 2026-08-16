//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 987/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk987<F: Float>(t3123: F, t3168: F, t3124: F, t3173: F, t11231: F, t4806: F, t1042: F, t1065: F, t675: F, t247: F, t906: F, t1063: F) -> (F, F, F, F) {
    let t11977 = t3123 * t3168;
    let t11980 = t3124 * t3173;
    let t11982 = t4806 * t11231;
    let t11983 = t1042 * t11982;
    let t11986 = t675 * t1065;
    let t11988 = t247 * t11986 * t906;
    let t11989 = t1063 * t11988;
    (t11977, t11980, t11983, t11989)
}
