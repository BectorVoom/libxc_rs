//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1324/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1324<F: Float>(t11461: F, t2380: F, t6475: F, t11457: F, t3206: F, t926: F, t11451: F, t2099: F, t3235: F, t11153: F, t2411: F, t10070: F, t10075: F, t10221: F, t2381: F, t28111: F, t28113: F, t32019: F, t32026: F, t32029: F, t32032: F, t406: F, t758: F, t824: F, t8319: F, t8450: F, t919: F, t921: F, t923: F) -> F {
    let t32035 = t2380 * t6475 * t11461;
    let t32045 = t3206 * t926 * t11457;
    let t32050 = t3235 * t2099 * t11451;
    let t32052 = t2411 * t11153;
    let t32057 = -F::new(0.53100265402527852012e-1) * t32019 * t923 + F::new(0.64311027177104605458e-3) * t8450 * t406 * t10075 * t10070 + F::new(0.91464571985215438873e-2) * t32026 + F::new(0.85748036236139473947e-3) * t32029 + F::new(0.85748036236139473947e-3) * t32032 - F::new(0.85748036236139473947e-3) * t32035 + F::new(0.68598428988911579154e-2) * t8319 * t10221 - F::new(0.42874018118069736972e-3) * t2380 * t2381 * t11153 * t919 * t921 - F::new(0.42874018118069736972e-3) * t32045 - F::new(0.85748036236139473944e-3) * t28111 + F::new(0.91464571985215438873e-2) * t28113 + F::new(0.25724410870841842184e-2) * t32050 + F::new(0.12862205435420921092e-2) * t3235 * t758 * t32052 * t824;
    t32057
}
