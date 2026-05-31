//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1179/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1179<F: Float>(t25759: F, t4433: F, t119706: F, t119737: F, t119747: F, t125968: F, t125976: F, t125980: F, t126013: F, t126422: F, t127190: F, t127193: F, t127199: F, t127204: F, t127207: F, t127212: F, t127218: F, t127227: F, t1940: F, t2403: F, t25206: F, t25440: F, t27382: F, t27764: F, t27770: F, t27777: F, t27800: F, t31859: F, t33727: F, t33888: F, t7091: F, t7200: F, t7862: F, t7869: F, t8494: F) -> F {
    let t127233 = t25759 * t4433;
    let t127236 = F::cast_from(3.0_f64) * t119706 * t127190 - F::cast_from(3.0_f64) * t25206 * t127193 + F::cast_from(3.0_f64) * t126422 * t27764 + t125968 * t27800 - F::cast_from(3.0_f64) * t25206 * t127199 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t119747 * t27770 - F::cast_from(3.0_f64) * t125980 * t127204 - t1940 * t7091 * t127207 - t1940 * t25440 * t33888 - t1940 * t7091 * t127212 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t33727 * t7200 + F::cast_from(2.0_f64) * t27382 * t127218 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8494 * t27777 + t125976 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t31859 * t7862 - t1940 * t7091 * t127227 - t1940 * t119737 * t7869 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t126013 * t127233;
    t127236
}
