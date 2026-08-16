//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 499/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk499(t1572: f64, t1577: f64, t1579: f64, t1584: f64, t1588: f64, t1592: f64, t1595: f64, t1601: f64, t1608: f64, t1612: f64, t1619: f64, t1622: f64, t2130: f64, t2133: f64, t2136: f64, t2139: f64, t2142: f64, t2152: f64, t2159: f64, t2166: f64, t2169: f64, t2173: f64, t2178: f64, t2184: f64, t2187: f64, t2192: f64, t2257: f64, t535: f64, t566: f64, t568: f64, t574: f64, t576: f64) -> f64 {
    let t2259 = 0.54878743191129263322e-1_f64 * t535 * t1572 + 0.86682217400542685632e-1_f64 * t1577 * t1579 - 0.86682217400542685632e-1_f64 * t1584 * t576 - 0.43341108700271342816e-1_f64 * t574 * t1588 + 0.2600466522016280569e0_f64 * t1592 * t1595 + 0.12805040077930161442e0_f64 * t1601 + 0.10975748638225852664e-1_f64 * t1608 + 0.11643651550782197811e-1_f64 * t1612 - t1619 - t1622 + t2130 + 0.86682217400542685632e-1_f64 * t2133 * t2136 + 0.2600466522016280569e0_f64 * t2139 * t2142 - 0.11643651550782197811e-1_f64 * t2152 - 0.97574405393827830186e-2_f64 * t2159 - t2166 - 0.2600466522016280569e0_f64 * t2169 * t568 - 0.13002332610081402845e0_f64 * t566 * t2173 + 0.25610080155860322884e0_f64 * t2178 + 0.86682217400542685632e-1_f64 * t2184 * t2187 + 0.23115257973478049502e0_f64 * t2192 + t2257;
    t2259
}
