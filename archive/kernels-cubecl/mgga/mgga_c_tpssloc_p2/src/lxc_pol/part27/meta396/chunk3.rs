//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1622/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1622<F: Float>(t1137: F, t15117: F, t1147: F, t4832: F, t1687: F, t3400: F, t1156: F, t14829: F, t3375: F, t1129: F, t11356: F, t1148: F, t1157: F, t14840: F, t14847: F, t14849: F, t14852: F, t1695: F, t3371: F, t3378: F, t3396: F, t3404: F, t4835: F, t4858: F) -> F {
    let t15118 = t15117 * t1137;
    let t15121 = t4832 * t1147;
    let t15126 = t1687 * t3400;
    let t15133 = t14829 * t1156;
    let t15136 = t1687 * t3375;
    let t15139 = F::cast_from(1.0_f64) * t1129 * t15118 + F::cast_from(0.11696447245269292414e1_f64) * t15121 * t1157 + F::cast_from(0.5848223622634646207e0_f64) * t4835 * t3396 + F::cast_from(0.17315859105681463759e2_f64) * t15126 * t3404 + F::cast_from(0.5848223622634646207e0_f64) * t11356 * t1695 + F::cast_from(0.11696447245269292414e1_f64) * t3371 * t4858 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t15133 + t14840 - F::cast_from(0.11696447245269292414e1_f64) * t15136 * t3378 - t14847 - t14849 - t14852;
    t15139
}
