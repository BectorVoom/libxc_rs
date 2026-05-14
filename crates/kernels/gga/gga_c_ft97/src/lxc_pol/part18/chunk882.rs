//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 882/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk882<F: Float>(t1391: F, t1643: F, t2205: F, t1882: F, t5882: F, t1986: F, t23400: F, t28: F, t89: F, t23490: F, t9236: F, t1369: F, t91: F, t9252: F, t26: F, t1359: F, t2087: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23595 = t2205 * t1391 * t1643;
    let t23598 = t1882 * t5882;
    let t23600 = t23400 * t1986;
    let t23601 = t28 * t23600;
    let t23602 = t89 * t23601;
    let t23604 = t9236 * t23490;
    let t23606 = t1369 * t28 * t23604;
    let t23608 = t91 * t9252;
    let t23609 = t23608 * t26;
    let t23610 = t1359 * t2087;
    (t23595, t23598, t23600, t23602, t23604, t23606, t23608, t23609, t23610)
}
