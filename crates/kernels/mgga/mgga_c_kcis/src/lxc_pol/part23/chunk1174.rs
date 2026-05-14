//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1174/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1174<F: Float>(t18377: F, t1881: F, t2233: F, t27328: F, t27723: F, t637: F, t8130: F, t92165: F, t92168: F, t92170: F, t92339: F, t92344: F, t92351: F, t93817: F, t97584: F, t1640: F, t18367: F, t18378: F, t1884: F, t2272: F, t27717: F, t27720: F, t27726: F, t446: F, t448: F, t4504: F, t5406: F, t92356: F, t92360: F, t92368: F, t92375: F, t97601: F) -> (F, F) {
    let t99767 = t8130 * t27328 / 8.0 - t2233 * t18377 * t637 / 16.0 + t93817 - t92165 + t1881 * t27723 / 16.0 + t97584 + t92168 + t92170 + t92339 + t92344 - t92351;
    let t99786 = -t446 * t18378 * t2272 / 16.0 - t2233 * t5406 * t1640 / 8.0 + t1881 * t27726 / 8.0 - t2233 * t448 * t18367 / 16.0 + t92356 + t1881 * t27717 / 16.0 - t92360 + t92368 - t2233 * t1884 * t4504 / 16.0 + t8130 * t27720 / 16.0 - t92375 + t97601;
    (t99767, t99786)
}
