//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1036/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1036<F: Float>(t776: F, t857: F, t865: F, t23270: F, t22986: F, t25: F, t2749: F, t606: F, t868: F, t2745: F, t2379: F, t28: F) -> (F, F, F, F, F, F, F) {
    let t23272 = t857 * t776 * t865;
    let t23273 = t23270 * t23272;
    let t23274 = t22986 * t23273;
    let t23296 = t25 * t2749;
    let t23299 = t606 * t868;
    let t23302 = t25 * t2745;
    let t23781 = t28 * t2379;
    (t23272, t23273, t23274, t23296, t23299, t23302, t23781)
}
