//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1103/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1103<F: Float>(t1251: F, t20624: F, t15518: F, t15547: F, t15549: F, t20601: F, t20604: F, t20607: F, t20610: F, t20614: F, t20619: F, t3490: F, t3514: F, t6759: F, t6771: F, t18570: F, t5310: F) -> (F, F) {
    let t20625 = t1251 * t20624;
    let t20630 = -t3514 * t20601 / 432.0 - t3514 * t20604 / 72.0 + 7.0 / 1296.0 * t3514 * t20607 + t3514 * t20610 / 108.0 - t3514 * t20614 / 288.0 - t15518 + t1251 * t20619 / 96.0 - t3490 * t6771 / 216.0 + t20625 / 1728.0 - t3490 * t6759 / 162.0 - t15547 - t15549 / 1296.0;
    let t20632 = t5310 * t18570;
    (t20630, t20632)
}
