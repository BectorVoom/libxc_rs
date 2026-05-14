//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 798/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk798<F: Float>(t10024: F, t11823: F, t43881: F, t44707: F, t5241: F, t5640: F, t590: F, t11622: F, t2464: F, t2465: F, t825: F, t13641: F, t2013: F, t45466: F, t969: F, t32809: F, t32810: F, t45369: F) -> (F, F, F, F, F, F, F) {
    let t45678 = t11823 * t10024;
    let t45680 = 0.15337170381568299871e1 * t43881;
    let t45684 = 0.13803453343411469884e2 * t5640 * t5241 * t44707 * t590;
    let t45687 = t825 * t2464 * t2465 * t11622;
    let t45688 = 0.42603251059911944084e-1 * t45687;
    let t45689 = t2013 * t13641;
    let t45690 = 0.19171462976960374838e0 * t45689;
    let t45692 = t825 * t969 * t45466;
    let t45693 = 0.19171462976960374838e0 * t45692;
    let t45700 = 0.85801175884441024004e1 * t32809 * t32810 * t45369;
    (t45678, t45680, t45684, t45688, t45690, t45693, t45700)
}
