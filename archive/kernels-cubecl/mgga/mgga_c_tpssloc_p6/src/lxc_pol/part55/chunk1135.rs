//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1135/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1135<F: Float>(t1519: F, t213: F, t225: F, t794: F, t25051: F, t1509: F, t6624: F, t1902: F, t4233: F, t25161: F, t214: F, t4265: F) -> (F, F, F, F, F, F, F) {
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86988 = t25051 * t225;
    let t87567 = t6624 * t1509;
    let t87620 = t1902 * t4233;
    let t87758 = t25161 * t225;
    let t87782 = t214 * t4265;
    (t86873, t86893, t86988, t87567, t87620, t87758, t87782)
}
