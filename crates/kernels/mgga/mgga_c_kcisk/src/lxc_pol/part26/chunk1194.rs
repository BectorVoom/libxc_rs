//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1194/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1194<F: Float>(t34722: F, t34755: F, t34792: F, t34829: F, t504: F, t2282: F, t33618: F, t32229: F, t8189: F, t8286: F, t9483: F, t27047: F, t2732: F, t20922: F, t9831: F, t6241: F, t9848: F) -> (F, F, F, F, F, F, F, F) {
    let t34831 = t34722 + t34755 + t34792 + t34829;
    let t34832 = t34831 * t504;
    let t34834 = 2.0 * t33618 * t2282;
    let t34836 = 2.0 * t32229 * t8189;
    let t34837 = t9483 * t8286;
    let t34838 = t27047 * t2732;
    let t34840 = 4.0 * t20922 * t9831;
    let t34842 = 2.0 * t6241 * t9848;
    (t34831, t34832, t34834, t34836, t34837, t34838, t34840, t34842)
}
