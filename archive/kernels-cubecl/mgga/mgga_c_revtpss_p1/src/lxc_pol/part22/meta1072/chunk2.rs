//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3844/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3844<F: Float>(t1388: F, t1390: F, t1410: F, t3889: F, t4002: F, t4012: F, t46598: F, t46602: F, t46620: F, t46633: F, t46645: F, t6816: F, t73923: F, t73927: F, t73929: F, t73937: F, t73942: F, t828: F) -> F {
    let t73947 = -F::cast_from(0.76220476654346199061e-4_f64) * t46598 + F::cast_from(0.54208002996571016772e-3_f64) * t46602 + F::cast_from(0.14450132032386466905e-2_f64) * t46620 + F::cast_from(0.1133779590233399711e0_f64) * t46633 - F::cast_from(0.10276933901433255263e-1_f64) * t46645 - F::cast_from(0.57165357490759649296e-4_f64) * t73923 + F::cast_from(0.14291339372689912324e-4_f64) * t73927 + F::cast_from(0.11337795902333997111e-1_f64) * t73929 + F::cast_from(0.42874018118069736972e-2_f64) * t1410 * t4012 * t828 * t6816 * t3889 - F::cast_from(0.42874018118069736972e-3_f64) * t1388 * t1390 * t828 * t73937 + F::cast_from(0.85748036236139473944e-3_f64) * t4002 * t1390 * t828 * t73942;
    t73947
}
