//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1030/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1030<F: Float>(t19209: F, t829: F, t2894: F, t14518: F, t14527: F, t14529: F, t19190: F, t19194: F, t19197: F, t19200: F, t19203: F, t19206: F, t9883: F, t991: F, t9918: F, t1704: F, t4621: F) -> (F, F) {
    let t19210 = t19209 * t829;
    let t19211 = t2894 * t19210;
    let t19214 = t14518 - t14527 - t14529 / 648.0 + t9883 - t9918 / 1296.0 - t991 * t19190 / 144.0 - t19194 / 432.0 - t991 * t19197 / 216.0 - t991 * t19200 / 36.0 + 7.0 / 648.0 * t991 * t19203 + t991 * t19206 / 54.0 - t991 * t19211 / 288.0;
    let t19218 = t4621 * t1704;
    (t19214, t19218)
}
