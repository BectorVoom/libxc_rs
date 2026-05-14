//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 962/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk962<F: Float>(t20117: F, t6508: F, t20013: F, t1433: F, t9271: F, t1323: F, t874: F, t2366: F, t15478: F, t4779: F, t584: F, t1564: F, t40: F, t6509: F, t18821: F, t9439: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20513 = t6508 * t20117;
    let t20521 = t6508 * t20013;
    let t20535 = t1433 * t9271;
    let t20539 = t874 * t1323;
    let t20540 = t2366 * t20539;
    let t20549 = t584 * t4779 * t15478;
    let t20550 = t40 * t1564;
    let t20551 = t20550 * t6509;
    let t20555 = t584 * t18821;
    let t20556 = t9439 * t6509;
    (t20513, t20521, t20535, t20539, t20540, t20549, t20550, t20551, t20555, t20556)
}
