//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1048/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1048<F: Float>(t10119: F, t10127: F, t1349: F, t2268: F, t2343: F, t31522: F, t31525: F, t31527: F, t31533: F, t31534: F, t31539: F, t31542: F, t31546: F, t31551: F, t31553: F, t31556: F, t31558: F, t3818: F, t4323: F, t6313: F) -> (F,) {
    let t31562 = t31522 + t31525 + t31527 - 0.7588001769513639893e-1 * t3818 * t10119 + 0.7588001769513639893e-1 * t6313 * t10127 + t31533 + 0.56910013271352299198e-1 * t2268 * t2343 * t31534 + t31539 + t31542 + t31546 - t31551 + t31553 - t31556 - 0.63233348079280332442e-2 * t1349 * t4323 * t31558;
    (t31562,)
}
