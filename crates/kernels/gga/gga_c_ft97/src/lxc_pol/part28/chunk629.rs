//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 629/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk629<F: Float>(t1060: F, t574: F, t5842: F, t23571: F, t3455: F, t12968: F, t13153: F, t5856: F, t6626: F, t9419: F, t23581: F, t925: F, t2221: F, t3052: F, t5855: F, t157: F, t9016: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26978 = t574 * t1060 * t5842;
    let t26981 = t23571 * t3455;
    let t26982 = t12968 * t26981;
    let t26985 = t13153 * t5856;
    let t26988 = t9419 * t6626;
    let t26991 = t23581 * t925;
    let t26992 = t2221 * t26991;
    let t26995 = t5855 * t3052;
    let t26996 = t2221 * t26995;
    let t26999 = t9016 * t157;
    (t26978, t26981, t26982, t26985, t26988, t26991, t26992, t26995, t26996, t26999)
}
