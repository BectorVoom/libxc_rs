//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1243/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1243<F: Float>(t20670: F, t20671: F, t26922: F, t21283: F, t2890: F, t7035: F, t10402: F, t4811: F, t10409: F, t1441: F, t31557: F, t493: F, t3394: F, t6576: F, t6578: F, t1339: F) -> (F, F, F, F, F, F, F) {
    let t34873 = t20670 * t20671 * t26922;
    let t34874 = 0.85206502119823888168e-1 * t34873;
    let t34876 = t21283 * t2890 * t7035;
    let t34877 = 0.38342925953920749676e0 * t34876;
    let t34878 = t4811 * t10402;
    let t34879 = 0.51123901271894332902e0 * t34878;
    let t34880 = t1441 * t10409;
    let t34881 = 0.1022478025437886658e1 * t34880;
    let t34882 = t493 * t31557;
    let t34886 = t493 * t3394;
    let t34888 = t6576 * t34886 * t6578;
    let t34889 = 0.76685851907841499352e0 * t34888;
    let t34890 = t1339 * t3394;
    (t34874, t34877, t34879, t34881, t34882, t34889, t34890)
}
