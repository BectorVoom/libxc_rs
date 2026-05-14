//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 523/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk523<F: Float>(t1001: F, t4972: F, t286: F, t1700: F, t1706: F, t285: F, t2870: F, t2872: F, t2879: F, t2882: F, t2885: F, t4937: F, t4940: F, t4944: F, t4948: F, t4953: F, t4959: F, t4963: F, t4968: F, t984: F, t991: F) -> (F, F) {
    let t4973 = t1001 * t4972;
    let t4974 = t286 * t4973;
    let t4977 = -t2870 / 108.0 - t2879 + t2882 / 864.0 - t2885 / 288.0 - t2872 * t1700 / 108.0 + t4937 / 864.0 + t991 * t4940 / 216.0 - t991 * t4944 / 288.0 - t991 * t4948 / 144.0 + t991 * t4953 / 144.0 + t984 * t1706 / 36.0 - t4959 / 288.0 - t991 * t4963 / 288.0 + t991 * t4968 / 48.0 - t285 * t4974 / 96.0;
    (t4973, t4977)
}
