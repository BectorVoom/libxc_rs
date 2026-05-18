//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1002/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1002<F: Float>(t7979: F, t7982: F, t6177: F, t6256: F, t7970: F, t7973: F, t7975: F, t7986: F, t7990: F, t7994: F, t7997: F, t8000: F) -> F {
    let t8059 = F::new(0.41678e0) * t7979;
    let t8060 = F::new(0.41678e0) * t7982;
    let t8066 = -F::new(0.17648625e1) * t7970 + F::new(0.6311625e0) * t7973 + F::new(0.31558125e0) * t7975 - t6256 + F::new(0.69463333333333333333e0) * t6177 - t8059 - t8060 + F::new(0.312585e0) * t7986 + F::new(0.62517e0) * t7990 + F::new(0.312585e0) * t7994 + F::new(0.264729375e1) * t7997 - F::new(0.157790625e0) * t8000;
    t8066
}
