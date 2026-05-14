//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1285/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1285<F: Float>(t2157: F, t2179: F, t6718: F, t3483: F, t95021: F, t1349: F, t26552: F, t376: F, t1637: F, t6617: F, t1058: F, t13021: F, t13239: F, t1389: F, t1986: F, t2081: F, t2180: F, t23400: F, t24080: F, t24081: F, t24102: F, t27406: F, t28: F, t3450: F, t39653: F, t564: F, t5772: F, t6587: F, t6708: F, t6723: F, t94155: F, t9432: F, t9439: F) -> (F, F, F) {
    let t104584 = t2179 * t6718 * t2157;
    let t104586 = t95021 * t3483;
    let t104599 = 2.0 / 9.0 * t1349 * t376 * t26552;
    let t104619 = t1349 * t1637 * t6617;
    let t104621 = 4.0 * t104584 + 8.0 * t104586 - 2.0 * t564 * t27406 - 12.0 * t9439 * t6708 * t2157 + t1349 * t28 * t23400 * t1058 * t1986 + t104599 + 2.0 * t5772 * t9432 * t24102 * t3450 - t2081 * t6723 - t1349 * t28 * t94155 * t6587 / 3.0 + 48.0 * t39653 * t6708 * t2180 - 2.0 * t13239 * t1389 + t5772 * t24080 * t24081 * t13021 / 9.0 + 2.0 / 27.0 * t104619;
    (t104584, t104586, t104621)
}
