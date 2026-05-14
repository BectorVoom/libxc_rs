//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 724/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk724<F: Float>(t1960: F, t596: F, t1633: F, t8822: F, t1860: F, t3103: F, t3105: F, t1030: F, t3717: F, t1749: F, t1736: F, t3131: F, t1743: F, t3060: F, t3127: F, t3132: F, t5285: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9009 = t596 * t1960;
    let t9011 = t1633 * t8822;
    let t9013 = t1860 * t3103;
    let t9014 = t9013 * t3105;
    let t9016 = t1030 * t3717;
    let t9017 = t9016 * t1749;
    let t9019 = t3131 * t1736;
    let t9020 = t1743 * t9019;
    let t9021 = t9020 * t1749;
    let t9023 = t3060 * t3127;
    let t9024 = t9023 * t1749;
    let t9026 = t5285 * t3132;
    (t9009, t9011, t9014, t9016, t9017, t9019, t9020, t9021, t9024, t9026)
}
