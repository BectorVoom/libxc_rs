//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 825/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk825<F: Float>(t13287: F, t64: F, t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t2268: F, t35901: F, t894: F, t426: F, t44386: F, t535: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44592 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13287 * t64;
    let t44595 = F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t39624;
    let t44596 = F::cast_from(63.0_f64) / F::cast_from(8192.0_f64) * t39626;
    let t44597 = F::cast_from(63.0_f64) / F::cast_from(524288.0_f64) * t39632;
    let t44598 = F::cast_from(21.0_f64) / F::cast_from(524288.0_f64) * t39646;
    let t44599 = F::cast_from(21.0_f64) / F::cast_from(8192.0_f64) * t39648;
    let t44600 = F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t39650;
    let t44618 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t894 * t35901;
    let t44622 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t44386 * t426;
    (t44592, t44595, t44596, t44597, t44598, t44599, t44600, t44618, t44622)
}
