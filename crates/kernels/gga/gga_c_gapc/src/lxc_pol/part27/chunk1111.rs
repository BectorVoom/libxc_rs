//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1111/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1111<F: Float>(t297: F, t825: F, t10366: F, t891: F, t11636: F, t11688: F, t6948: F, t10102: F, t11620: F, t1062: F, t125: F, t2188: F, t2536: F, t329: F, t10357: F, t11674: F, t35734: F) -> (F, F, F, F, F, F) {
    let t35846 = t825 * t297;
    let t35848 = t10366 * t35846 * t891;
    let t35851 = t11636 * t6948 * t11688;
    let t35853 = t10102 * t11620;
    let t35858 = t1062 * t125 * t2188 * t329 * t2536;
    let t35861 = t35734 * t11674 * t10357;
    (t35846, t35848, t35851, t35853, t35858, t35861)
}
