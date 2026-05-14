//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 940/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk940<F: Float>(t6177: F, t6256: F, t7970: F, t7973: F, t7975: F, t7986: F, t7990: F, t7994: F, t7997: F, t8000: F, t8059: F, t8060: F, t8054: F, t871: F, t1201: F, t2295: F) -> (F, F, F) {
    let t8066 = -0.17648625e1 * t7970 + 0.6311625e0 * t7973 + 0.31558125e0 * t7975 - t6256 + 0.69463333333333333333e0 * t6177 - t8059 - t8060 + 0.312585e0 * t7986 + 0.62517e0 * t7990 + 0.312585e0 * t7994 + 0.264729375e1 * t7997 - 0.157790625e0 * t8000;
    let t8067 = t8054 + t8066;
    let t8068 = t8067 * t871;
    let t8071 = t1201 * t2295;
    (t8067, t8068, t8071)
}
