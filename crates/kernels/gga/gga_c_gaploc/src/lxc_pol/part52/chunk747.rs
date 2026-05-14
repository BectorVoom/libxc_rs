//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 747/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk747<F: Float>(t2679: F, t3621: F, t9796: F, t1029: F, t10827: F, t13657: F, t4614: F, t833: F, t11784: F, t2617: F, t7810: F, t44713: F, t4820: F, t7513: F, t11780: F, t23000: F) -> (F, F, F, F, F, F) {
    let t45219 = t9796 * t3621 * t2679;
    let t45222 = t9796 * t1029 * t10827;
    let t45226 = 0.58281247449959539508e2 * t833 * t4614 * t13657;
    let t45228 = t7810 * t11784 * t2617;
    let t45229 = 0.19171462976960374838e0 * t45228;
    let t45232 = 0.79445533226334281487e-1 * t7513 * t4820 * t44713;
    let t45234 = t23000 * t11780 * t2617;
    (t45219, t45222, t45226, t45229, t45232, t45234)
}
