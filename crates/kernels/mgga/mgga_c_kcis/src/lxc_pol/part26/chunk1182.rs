//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1182/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1182<F: Float>(t1014: F, t29372: F, t27475: F, t303: F, t6932: F, t1459: F, t6923: F, t1394: F, t20878: F, t7923: F, t20883: F, t21510: F, t27607: F, t28727: F, t28811: F, t29533: F, t95168: F, t99630: F, t99639: F, t99644: F) -> (F, F, F, F, F, F, F) {
    let t102674 = t1014 * t29372;
    let t102678 = t303 * t27475 * t6932;
    let t102681 = t303 * t1459 * t6923;
    let t102684 = t1394 * t7923 * t20878;
    let t102687 = t1394 * t7923 * t20883;
    let t102694 = t1394 * t7923 * t21510;
    let t102696 = -t95168 - t99630 + 0.11607361111111111111e-2 * t102674 + 0.15445601851851851852e-3 * t99639 + 0.92858888888888888886e-2 * t102678 + t99644 - 0.38691203703703703703e-3 * t102681 + 0.77382407407407407407e-3 * t102684 - 0.23214722222222222222e-2 * t102687 + 0.23168402777777777778e-3 * t27607 * t29533 + 0.37069444444444444444e-2 * t28727 * t28811 + 0.77382407407407407407e-3 * t102694;
    (t102674, t102678, t102681, t102684, t102687, t102694, t102696)
}
