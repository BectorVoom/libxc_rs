//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1701/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1701<F: Float>(t1343: F, t13656: F, t1448: F, t198: F, t3828: F, t3829: F, t3889: F, t39419: F, t39422: F, t46280: F, t46282: F, t46287: F, t46290: F, t46292: F, t46297: F, t46298: F, t46303: F, t46304: F, t46345: F, t5536: F, t5541: F, t9547: F) -> F {
    let t46349 = F::new(3.0) * t1343 * t198 * t46345 + F::new(36.0) * t13656 * t198 * t3889 - F::new(4.0) * t1448 * t46304 * t5541 + F::new(18.0) * t198 * t3828 * t46298 + F::new(36.0) * t3829 * t5536 * t9547 - t39419 - t39422 + t46280 + t46282 - t46287 + t46290 - t46292 - t46297 + t46303;
    t46349
}
