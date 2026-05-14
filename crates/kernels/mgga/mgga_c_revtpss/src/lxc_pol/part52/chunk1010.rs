//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1010/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1010<F: Float>(t120980: F, t1372: F, t32265: F, t124: F, t1380: F, t1444: F, t800: F, t32705: F, t239: F, t8583: F, t8589: F, t9990: F, t1405: F, t32272: F, t32269: F, t3974: F) -> (F, F, F, F, F, F, F) {
    let t120981 = t120980 * t1372;
    let t120982 = t32265 * t120981;
    let t120983 = 0.14874931683620404328e-3 * t120982;
    let t120986 = t1380 * t800 * t124 * t1444;
    let t120987 = t32705 * t120986;
    let t120988 = 0.7437465841810202164e-4 * t120987;
    let t120991 = t8583 * t8589 * t9990 * t239;
    let t120994 = t32272 * t1405;
    let t120995 = 0.17354086964223805049e-2 * t120994;
    let t120996 = t32269 * t3974;
    (t120981, t120983, t120986, t120988, t120991, t120995, t120996)
}
