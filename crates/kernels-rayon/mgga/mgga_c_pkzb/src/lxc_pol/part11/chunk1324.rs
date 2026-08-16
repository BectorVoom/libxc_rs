//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1324/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1324(t11461: f64, t2380: f64, t6475: f64, t11457: f64, t3206: f64, t926: f64, t11451: f64, t2099: f64, t3235: f64, t11153: f64, t2411: f64, t10070: f64, t10075: f64, t10221: f64, t2381: f64, t28111: f64, t28113: f64, t32019: f64, t32026: f64, t32029: f64, t32032: f64, t406: f64, t758: f64, t824: f64, t8319: f64, t8450: f64, t919: f64, t921: f64, t923: f64) -> f64 {
    let t32035 = t2380 * t6475 * t11461;
    let t32045 = t3206 * t926 * t11457;
    let t32050 = t3235 * t2099 * t11451;
    let t32052 = t2411 * t11153;
    let t32057 = -0.53100265402527852012e-1_f64 * t32019 * t923 + 0.64311027177104605458e-3_f64 * t8450 * t406 * t10075 * t10070 + 0.91464571985215438873e-2_f64 * t32026 + 0.85748036236139473947e-3_f64 * t32029 + 0.85748036236139473947e-3_f64 * t32032 - 0.85748036236139473947e-3_f64 * t32035 + 0.68598428988911579154e-2_f64 * t8319 * t10221 - 0.42874018118069736972e-3_f64 * t2380 * t2381 * t11153 * t919 * t921 - 0.42874018118069736972e-3_f64 * t32045 - 0.85748036236139473944e-3_f64 * t28111 + 0.91464571985215438873e-2_f64 * t28113 + 0.25724410870841842184e-2_f64 * t32050 + 0.12862205435420921092e-2_f64 * t3235 * t758 * t32052 * t824;
    t32057
}
