//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 329/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk329<F: Float>(t1725: F, t425: F, t47: F, t625: F, t68: F, t72: F, t173: F, t424: F, t419: F, t23: F, t358: F) -> (F, F, F, F, F, F) {
    let t1726 = t1725 * t425;
    let t1728 = t47 * t625;
    let t1730 = t68 * t1728 * t72;
    let t1731 = 0.42562405586419753087e-2 * t1730;
    let t1732 = t173 * t424;
    let t1733 = t419 * t1732;
    let t1736 = 1.0 / t23 / t358;
    (t1726, t1730, t1731, t1732, t1733, t1736)
}
