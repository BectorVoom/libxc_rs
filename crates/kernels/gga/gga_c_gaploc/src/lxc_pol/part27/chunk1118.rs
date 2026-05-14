//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1118/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1118<F: Float>(t10924: F, t5679: F, t6096: F, t11069: F, t5669: F, t20671: F, t25070: F, t28856: F, t11029: F, t2087: F, t4614: F, t10951: F, t5782: F, t1890: F, t3487: F, t7805: F, t7810: F) -> (F, F, F, F, F, F) {
    let t33269 = 0.71500979903700853338e0 * t5679 * t10924 * t6096;
    let t33271 = 0.2044956050875773316e1 * t5669 * t11069;
    let t33273 = t28856 * t20671 * t25070;
    let t33274 = 0.2556195063594716645e0 * t33273;
    let t33282 = 0.18404604457881959845e2 * t2087 * t4614 * t11029;
    let t33284 = 0.18404604457881959845e2 * t5782 * t10951;
    let t33289 = t1890 * t3487;
    let t33291 = t7810 * t33289 * t7805;
    (t33269, t33271, t33274, t33282, t33284, t33291)
}
