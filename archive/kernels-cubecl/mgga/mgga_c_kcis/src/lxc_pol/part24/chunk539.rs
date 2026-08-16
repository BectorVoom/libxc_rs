//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 539/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk539<F: Float>(t1001: F, t4972: F, t286: F, t1700: F, t1706: F, t285: F, t2870: F, t2872: F, t2879: F, t2882: F, t2885: F, t4937: F, t4940: F, t4944: F, t4948: F, t4953: F, t4959: F, t4963: F, t4968: F, t984: F, t991: F) -> (F, F) {
    let t4973 = t1001 * t4972;
    let t4974 = t286 * t4973;
    let t4977 = -t2870 / F::cast_from(108.0_f64) - t2879 + t2882 / F::cast_from(864.0_f64) - t2885 / F::cast_from(288.0_f64) - t2872 * t1700 / F::cast_from(108.0_f64) + t4937 / F::cast_from(864.0_f64) + t991 * t4940 / F::cast_from(216.0_f64) - t991 * t4944 / F::cast_from(288.0_f64) - t991 * t4948 / F::cast_from(144.0_f64) + t991 * t4953 / F::cast_from(144.0_f64) + t984 * t1706 / F::cast_from(36.0_f64) - t4959 / F::cast_from(288.0_f64) - t991 * t4963 / F::cast_from(288.0_f64) + t991 * t4968 / F::cast_from(48.0_f64) - t285 * t4974 / F::cast_from(96.0_f64);
    (t4973, t4977)
}
