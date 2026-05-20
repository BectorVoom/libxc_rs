//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1919/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1919<F: Float>(t103067: F, t2467: F, t1580: F, t26446: F, t689: F, t28368: F, t93321: F, t93374: F, t26544: F, t27216: F, t26506: F, t27213: F) -> (F, F, F, F, F, F) {
    let t103069 = F::cast_from(0.19514881078765566038e-1_f64) * t103067 * t2467;
    let t103072 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t26446 * t1580;
    let t103086 = F::cast_from(0.14456046980341999104e-1_f64) * t93321 * t28368;
    let t103088 = F::cast_from(0.25702851531048074406e-1_f64) * t93374 * t28368;
    let t103103 = F::cast_from(0.25702851531048074406e-1_f64) * t27216 * t26544;
    let t103114 = t27213 * t26506;
    (t103069, t103072, t103086, t103088, t103103, t103114)
}
