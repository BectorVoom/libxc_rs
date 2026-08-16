//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2847/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2847<F: Float>(t76911: F, t76929: F, t150: F, t190: F, t162: F, t187: F, t61020: F, t49866: F, t39423: F, t39425: F, t39433: F, t39438: F) -> (F, F, F, F, F, F, F, F) {
    let t76930 = t76911 + t76929;
    let t76932 = t150 * t76930 * t190;
    let t76935 = F::cast_from(0.19751673498613801407e-1_f64) * t76930 * t162 * t187;
    let t76936 = F::cast_from(36.0_f64) * t61020;
    let t76937 = F::cast_from(0.30762056574649219972e4_f64) * t49866;
    let t76938 = F::cast_from(0.21687162600603479684e-1_f64) * t39423;
    let t76939 = F::cast_from(0.32530743900905219526e-1_f64) * t39425;
    let t76940 = F::cast_from(0.48159733137676571078e0_f64) * t39433;
    let t76941 = F::cast_from(0.16265371950452609763e-1_f64) * t39438;
    (t76932, t76935, t76936, t76937, t76938, t76939, t76940, t76941)
}
