//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 830/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk830<F: Float>(t25253: F, t875: F, t296: F, t10666: F, t1501: F, t1508: F, t2409: F, t835: F, t1476: F, t2844: F, t2843: F, t840: F, t2842: F) -> (F, F, F, F, F, F, F, F) {
    let t25254 = t25253 * t875;
    let t25255 = t296 * t25254;
    let t25258 = t10666 * t1501;
    let t25259 = t296 * t25258;
    let t25263 = t835 * t1508 * t2409;
    let t25266 = t1476 * t2844;
    let t25268 = t840 * t2843 * t25266;
    let t25271 = t2842 * t1501;
    (t25254, t25255, t25258, t25259, t25263, t25266, t25268, t25271)
}
