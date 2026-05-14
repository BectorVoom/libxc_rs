//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 711/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk711<F: Float>(t2844: F, t359: F, t4547: F, t3210: F, t4554: F, t1754: F, t3255: F, t3262: F, t347: F, t1646: F) -> (F, F, F, F, F, F, F) {
    let t4555 = t359 * t2844;
    let t4556 = t4555 * t4547;
    let t4557 = t3210 * t4556;
    let t4558 = t4554 * t4557;
    let t4563 = t3255 * t1754;
    let t4565 = t3262 * t347;
    let t4566 = t2844 * t1646;
    (t4555, t4556, t4557, t4558, t4563, t4565, t4566)
}
