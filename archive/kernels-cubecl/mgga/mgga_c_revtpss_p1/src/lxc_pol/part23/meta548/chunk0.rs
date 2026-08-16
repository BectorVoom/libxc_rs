//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2099/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2099<F: Float>(t22190: F, t22203: F, t22210: F, t22220: F, t225: F, t1877: F, t73: F, t4010: F, t6836: F, t1353: F, t5591: F, t5651: F) -> (F, F, F, F, F) {
    let t22223 = (t22190 + t22203 + t22210 + t22220) * t225;
    let t22229 = t1877 * t73;
    let t22236 = t4010 * t6836;
    let t22237 = t22236 * t1353;
    let t22240 = t5651 * t5591;
    (t22223, t22229, t22236, t22237, t22240)
}
