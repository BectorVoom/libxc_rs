//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 498/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk498<F: Float>(t1572: F, t1577: F, t1579: F, t1584: F, t1588: F, t1592: F, t1595: F, t1601: F, t1608: F, t1612: F, t1619: F, t1622: F, t2130: F, t2133: F, t2136: F, t2139: F, t2142: F, t2152: F, t2159: F, t2166: F, t2169: F, t2173: F, t2178: F, t2184: F, t2187: F, t2192: F, t2257: F, t535: F, t566: F, t568: F, t574: F, t576: F) -> F {
    let t2259 = F::new(0.54878743191129263322e-1) * t535 * t1572 + F::new(0.86682217400542685632e-1) * t1577 * t1579 - F::new(0.86682217400542685632e-1) * t1584 * t576 - F::new(0.43341108700271342816e-1) * t574 * t1588 + F::new(0.2600466522016280569e0) * t1592 * t1595 + F::new(0.12805040077930161442e0) * t1601 + F::new(0.10975748638225852664e-1) * t1608 + F::new(0.11643651550782197811e-1) * t1612 - t1619 - t1622 + t2130 + F::new(0.86682217400542685632e-1) * t2133 * t2136 + F::new(0.2600466522016280569e0) * t2139 * t2142 - F::new(0.11643651550782197811e-1) * t2152 - F::new(0.97574405393827830186e-2) * t2159 - t2166 - F::new(0.2600466522016280569e0) * t2169 * t568 - F::new(0.13002332610081402845e0) * t566 * t2173 + F::new(0.25610080155860322884e0) * t2178 + F::new(0.86682217400542685632e-1) * t2184 * t2187 + F::new(0.23115257973478049502e0) * t2192 + t2257;
    t2259
}
