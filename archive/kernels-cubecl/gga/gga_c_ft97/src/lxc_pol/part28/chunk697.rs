//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 697/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk697<F: Float>(t27103: F, t446: F, t27064: F, t9073: F, t1369: F, t376: F, t6665: F, t27066: F, t27070: F, t27075: F, t27079: F, t27084: F, t27089: F, t27094: F, t27098: F, t27101: F) -> (F, F, F, F, F) {
    let t27104 = t446 * t27103;
    let t27106 = t9073 * t27064;
    let t27107 = t446 * t27106;
    let t27110 = t1369 * t376 * t6665;
    let t27112 = -t27066 / F::cast_from(3.0_f64) - t27070 / F::cast_from(3.0_f64) + t27075 / F::cast_from(9.0_f64) - t27079 / F::cast_from(12.0_f64) - t27084 / F::cast_from(12.0_f64) + t27089 / F::cast_from(4.0_f64) + t27094 / F::cast_from(4.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27098 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27101 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t27104 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27107 - t27110 / F::cast_from(3.0_f64);
    (t27104, t27106, t27107, t27110, t27112)
}
