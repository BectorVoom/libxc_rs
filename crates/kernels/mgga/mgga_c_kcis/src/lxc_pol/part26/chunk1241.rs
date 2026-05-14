//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1241/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1241<F: Float>(t101750: F, t101757: F, t103934: F, t103935: F, t1299: F, t1885: F, t2132: F, t2233: F, t28325: F, t28876: F, t28880: F, t446: F, t449: F, t5406: F, t7570: F, t8130: F, t92356: F, t92360: F, t92368: F, t92375: F, t99834: F) -> (F,) {
    let t103953 = -t446 * t449 * (t103934 + t103935) / 16.0 - t446 * t1885 * t28876 / 8.0 + t99834 + t8130 * t28880 / 8.0 + t92356 - t92360 - t2233 * t5406 * t2132 / 8.0 + t92368 - t2233 * t1299 * t7570 / 16.0 + t101750 - t92375 + t101757 + t8130 * t28325 / 8.0;
    (t103953,)
}
