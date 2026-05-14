//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 572/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk572<F: Float>(t3626: F, t70: F, t40: F, t6: F, t12: F, t171: F, t341: F, t630: F, t343: F, t122: F, t2252: F, t342: F, t657: F, t173: F, t703: F) -> (F, F, F, F, F, F, F) {
    let t8715 = t3626 * t70;
    let t8946 = t6 / t40;
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    let t8959 = t341 * t630;
    let t8963 = t341 * t343 * t70;
    let t8991 = t122 * t122;
    let t9482 = t342 * t2252 * t657 / 18.0;
    let t9483 = t173 * t703;
    (t8715, t8948, t8959, t8963, t8991, t9482, t9483)
}
