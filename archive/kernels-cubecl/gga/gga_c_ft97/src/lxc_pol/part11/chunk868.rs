//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 868/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk868<F: Float>(t1725: F, t8109: F, t2248: F, t419: F, t424: F, t67: F, t8063: F, t9: F, t425: F, t1732: F, t8130: F, t1736: F, t7763: F) -> (F, F, F, F, F) {
    let t37778 = t1725 * t8109;
    let t37781 = t419 * t2248 * t424;
    let t37784 = t9 * t67 * t8063;
    let t37785 = t37784 * t425;
    let t37787 = t8130 * t1732;
    let t37789 = t1736 * t7763;
    (t37778, t37781, t37785, t37787, t37789)
}
