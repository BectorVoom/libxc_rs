//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1237/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1237<F: Float>(t11587: F, t27940: F, t2993: F, t11604: F, t27868: F, t33748: F, t8843: F, t33152: F, t9256: F, t26034: F, t35050: F, t33373: F, t5395: F, t5974: F) -> (F, F, F, F, F, F) {
    let t35266 = t2993 * t11587 * t27940;
    let t35269 = t11604 * t27868;
    let t35272 = t2993 * t33748 * t8843;
    let t35275 = t2993 * t33152 * t9256;
    let t35277 = t35050 * t26034;
    let t35280 = t5395 * t33373 * t5974;
    (t35266, t35269, t35272, t35275, t35277, t35280)
}
