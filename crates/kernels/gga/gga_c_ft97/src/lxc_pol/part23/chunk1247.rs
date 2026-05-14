//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1247/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1247<F: Float>(t18724: F, t6135: F, t446: F, t9770: F, t1434: F, t31037: F, t681: F, t31030: F, t122003: F, t122471: F, t193: F, t2506: F, t122330: F, t2354: F, t110103: F, t123941: F, t123945: F, t123949: F, t123952: F, t123955: F) -> (F, F, F, F, F, F, F, F) {
    let t123957 = t6135 * t18724;
    let t123959 = t446 * t9770 * t123957;
    let t123962 = t1434 * t681 * t31037;
    let t123965 = t1434 * t681 * t31030;
    let t123968 = t446 * t9770 * t122003;
    let t123972 = t1434 * t193 * t2506 * t122471;
    let t123975 = t446 * t2354 * t122330;
    let t123977 = 2.0 / 3.0 * t123941 + t123945 / 9.0 + t110103 + t123949 / 18.0 + 8.0 / 9.0 * t123952 - 8.0 / 27.0 * t123955 - 4.0 / 9.0 * t123959 + t123962 / 3.0 - t123965 / 9.0 - 4.0 / 9.0 * t123968 + 2.0 / 3.0 * t123972 + 2.0 / 3.0 * t123975;
    (t123957, t123959, t123962, t123965, t123968, t123972, t123975, t123977)
}
