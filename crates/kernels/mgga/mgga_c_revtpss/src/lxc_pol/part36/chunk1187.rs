//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1187/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1187<F: Float>(t22914: F, t7264: F, t22865: F, t25983: F, t108587: F, t108590: F, t108592: F, t108601: F, t94477: F, t94484: F, t94523: F, t94526: F, t98218: F, t98220: F, t98224: F, t98260: F) -> (F,) {
    let t114564 = t7264 * t22914;
    let t114566 = t25983 * t22865;
    let t114570 = -t94477 - 0.18292914397043087774e-2 * t98218 + 0.17149607247227894789e-3 * t108587 - 0.27107389498472794076e-4 * t98220 - 0.12004725073059526352e-1 * t108590 + 0.60023625365297631762e-2 * t108592 - 0.34013387707001991332e-1 * t98224 + t94484 - 0.42874018118069736972e-3 * t114564 + 0.25724410870841842183e-2 * t114566 - 35.0 / 72.0 * t98260 - t94523 + t94526 + 0.42874018118069736972e-4 * t108601;
    (t114570,)
}
