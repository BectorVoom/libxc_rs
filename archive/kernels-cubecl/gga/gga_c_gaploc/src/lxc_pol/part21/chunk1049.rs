//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1049/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1049<F: Float>(t15478: F, t4779: F, t584: F, t1564: F, t40: F, t6509: F, t18821: F, t9439: F, t18362: F, t9448: F, t1397: F, t6823: F) -> (F, F, F, F, F, F, F, F) {
    let t20549 = t584 * t4779 * t15478;
    let t20550 = t40 * t1564;
    let t20551 = t20550 * t6509;
    let t20555 = t584 * t18821;
    let t20556 = t9439 * t6509;
    let t20560 = t584 * t18362;
    let t20561 = t9448 * t6509;
    let t20565 = t1397 * t6823;
    (t20549, t20550, t20551, t20555, t20556, t20560, t20561, t20565)
}
