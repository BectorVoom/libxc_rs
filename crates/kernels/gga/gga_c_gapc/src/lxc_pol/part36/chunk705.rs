//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 705/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk705<F: Float>(t5977: F, t9103: F, t1679: F, t3016: F, t3013: F, t5252: F, t3012: F, t5248: F, t1643: F, t116: F, t5312: F, t3708: F, t5407: F, t676: F, t8986: F, t5260: F) -> (F, F, F, F, F, F, F) {
    let t9104 = t5977 * t9103;
    let t9106 = t3016 * t1679;
    let t9108 = t5252 * t3013;
    let t9110 = t3012 * t5248;
    let t9111 = t1643 * t9110;
    let t9113 = t116 * t5312;
    let t9114 = t3708 * t5407;
    let t9115 = t9113 * t9114;
    let t9117 = t8986 * t676;
    let t9118 = t5260 * t9117;
    (t9104, t9106, t9108, t9111, t9113, t9115, t9118)
}
