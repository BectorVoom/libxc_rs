//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1033/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1033<F: Float>(t100981: F, t27384: F, t1113: F, t7782: F, t1711: F, t7086: F, t125961: F, t27799: F, t27363: F, t33: F, t25759: F, t4433: F, t119706: F, t119737: F, t119747: F, t125968: F, t125976: F, t125980: F, t126013: F, t126422: F, t127190: F, t127193: F, t127199: F, t1940: F, t2403: F, t25206: F, t25440: F, t27382: F, t27764: F, t27770: F, t27777: F, t27800: F, t31859: F, t33727: F, t33888: F, t7091: F, t7200: F, t7862: F, t7869: F, t8494: F) -> (F,) {
    let t127204 = t100981 * t27384;
    let t127207 = t1113 * t7782;
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    let t127233 = t25759 * t4433;
    let t127236 = 3.0 * t119706 * t127190 - 3.0 * t25206 * t127193 + 3.0 * t126422 * t27764 + t125968 * t27800 - 3.0 * t25206 * t127199 - 3.0 / 2.0 * t119747 * t27770 - 3.0 * t125980 * t127204 - t1940 * t7091 * t127207 - t1940 * t25440 * t33888 - t1940 * t7091 * t127212 + 3.0 / 2.0 * t2403 * t33727 * t7200 + 2.0 * t27382 * t127218 - 3.0 / 2.0 * t2403 * t8494 * t27777 + t125976 + 3.0 / 2.0 * t2403 * t31859 * t7862 - t1940 * t7091 * t127227 - t1940 * t119737 * t7869 / 2.0 - 3.0 * t126013 * t127233;
    (t127236,)
}
