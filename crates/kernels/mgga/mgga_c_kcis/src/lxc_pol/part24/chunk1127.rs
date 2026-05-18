//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1127/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1127<F: Float>(t1646: F, t4972: F, t1003: F, t6272: F, t167: F, t1704: F, t6276: F, t6544: F, t9985: F, t2835: F, t6432: F, t1141: F, t19824: F) -> (F, F, F, F, F, F, F) {
    let t71184 = t1646 * t4972;
    let t71203 = t6272 * t1003;
    let t71215 = t167 * t1704;
    let t71387 = t6276 * t1003;
    let t71722 = t6544 * t9985;
    let t71731 = t6432 * t2835;
    let t71840 = t19824 * t1141;
    (t71184, t71203, t71215, t71387, t71722, t71731, t71840)
}
