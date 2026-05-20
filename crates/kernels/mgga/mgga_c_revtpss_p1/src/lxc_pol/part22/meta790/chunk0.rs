//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2881/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2881<F: Float>(t39552: F, t562: F, t560: F, t9655: F, t225: F, t3896: F, t39515: F, t3900: F, t9292: F, t1419: F, t9646: F, t9648: F) -> (F, F, F, F, F) {
    let t46359 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t562;
    let t46361 = F::new(1.0) / t9655 / t560;
    let t46362 = t225 * t46361;
    let t46368 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t3896;
    let t46369 = t9292 * t3900;
    let t46378 = t9646 * t1419 * t9648;
    (t46359, t46362, t46368, t46369, t46378)
}
