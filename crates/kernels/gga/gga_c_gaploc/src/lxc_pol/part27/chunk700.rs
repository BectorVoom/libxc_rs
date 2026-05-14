//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 700/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk700<F: Float>(t531: F, t6474: F, t2349: F, t590: F, t1339: F, t2293: F, t107: F, t6514: F, t544: F, t1421: F, t2389: F, t4494: F, t901: F, t4502: F, t1415: F, t4390: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6937 = t531 * t6474;
    let t6944 = t2349 * t590;
    let t6949 = t1339 * t2293;
    let t6950 = t6949 * t590;
    let t6953 = t6514 * t107;
    let t6954 = t544 * t6953;
    let t6957 = t1421 * t2389;
    let t6959 = t4494 * t901;
    let t6961 = t4502 * t901;
    let t6963 = t1415 * t4390;
    (t6937, t6944, t6950, t6953, t6954, t6957, t6959, t6961, t6963)
}
