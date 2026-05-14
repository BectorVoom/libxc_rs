//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1044/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1044<F: Float>(t10242: F, t1624: F, t2268: F, t10116: F, t6305: F, t6313: F, t23726: F, t3355: F, t10256: F, t590: F, t23759: F, t10241: F, t161: F, t1353: F, t23767: F, t1061: F, t424: F, t481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31525 = 0.28455006635676149599e-1 * t2268 * t1624 * t10242;
    let t31527 = 0.56910013271352299198e-1 * t6305 * t10116;
    let t31533 = 0.7588001769513639893e-1 * t6313 * t10116;
    let t31539 = 0.15176003539027279786e0 * t23726 * t3355;
    let t31540 = t590 * t10256;
    let t31542 = 0.12646669615856066488e-1 * t23759 * t31540;
    let t31543 = t10241 * t161;
    let t31546 = 0.63233348079280332442e-2 * t23767 * t31543 * t1353;
    let t31548 = t481 * t1061 * t424;
    (t31525, t31527, t31533, t31539, t31540, t31542, t31543, t31546, t31548)
}
