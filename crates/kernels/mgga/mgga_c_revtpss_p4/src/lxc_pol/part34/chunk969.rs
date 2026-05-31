//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 969/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk969<F: Float>(t225: F, t23185: F, t23187: F, t23192: F, t23224: F, t10626: F, t23114: F, t4416: F, t5962: F, t23148: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t4415: F, t6006: F, t6010: F, t6013: F) -> F {
    let t23227 = (t23185 + t23187 + t23192 + t23224) * t225;
    let t23235 = t10626 * t23114;
    let t23238 = t4416 * t5962;
    let t23241 = t832 * t23148;
    let t23244 = -F::cast_from(36.0_f64) * t1553 * t6010 + F::cast_from(9.0_f64) * t1553 * t6013 + F::cast_from(9.0_f64) * t1555 * t6006 + F::cast_from(60.0_f64) * t227 * t23235 + F::cast_from(3.0_f64) * t227 * t23241 - t229 * t23227 - F::cast_from(36.0_f64) * t23238 * t4415;
    t23244
}
