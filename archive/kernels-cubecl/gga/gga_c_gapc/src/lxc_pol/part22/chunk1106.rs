//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1106/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1106<F: Float>(t1457: F, t632: F, t1266: F, t4048: F, t424: F, t116: F, t14873: F, t3116: F, t4687: F, t102: F, t5390: F, t8959: F) -> (F, F, F, F, F, F) {
    let t25514 = t632 * t1457;
    let t25526 = t1266 * t1457;
    let t25530 = t424 * t4048;
    let t25708 = t116 * t14873;
    let t25756 = t3116 * t4687;
    let t25813 = t8959 * t5390 * t102;
    (t25514, t25526, t25530, t25708, t25756, t25813)
}
