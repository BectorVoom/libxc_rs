//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 655/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk655<F: Float>(t273: F, t57: F, t1509: F, t424: F, t41: F, t1477: F, t1485: F, t1483: F, t400: F, t1384: F, t1409: F, t452: F) -> (F, F, F, F, F) {
    let t4145 = t57 * t273;
    let t4694 = t424 * t1509;
    let t4695 = t41 * t4694;
    let t4700 = t1477 * t1485;
    let t4702 = t1483 * t4700 * t400;
    let t4703 = F::new(0.48245938496077605201e2) * t4702;
    let t4704 = t1409 * t1384;
    let t4705 = t4704 * t452;
    (t4145, t4695, t4703, t4704, t4705)
}
