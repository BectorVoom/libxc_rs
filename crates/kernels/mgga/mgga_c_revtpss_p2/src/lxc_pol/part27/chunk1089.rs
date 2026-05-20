//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1089/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1089<F: Float>(t25234: F, t2677: F, t1941: F, t243: F, t2732: F, t2712: F, t64: F, t2710: F, t826: F, t2482: F, t27: F, t7036: F) -> (F, F, F, F, F) {
    let t25235 = t25234 * t2677;
    let t25236 = F::cast_from(0.2032800112371413129e-3_f64) * t25235;
    let t25237 = t1941 * t243;
    let t25238 = t25237 * t2732;
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25243 = F::cast_from(0.90357964994909313586e-5_f64) * t25242;
    let t25245 = t2482 * t7036 * t27;
    (t25236, t25238, t25240, t25243, t25245)
}
