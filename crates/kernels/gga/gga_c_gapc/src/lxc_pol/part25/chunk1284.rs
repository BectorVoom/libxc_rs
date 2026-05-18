//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1284/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1284<F: Float>(t2993: F, t33152: F, t9256: F, t26034: F, t35050: F, t33373: F, t5395: F, t5974: F, t1030: F, t9262: F, t11357: F, t26102: F) -> (F, F, F, F, F) {
    let t35275 = t2993 * t33152 * t9256;
    let t35277 = t35050 * t26034;
    let t35280 = t5395 * t33373 * t5974;
    let t35283 = t1030 * t33152 * t9262;
    let t35285 = t11357 * t26102;
    (t35275, t35277, t35280, t35283, t35285)
}
