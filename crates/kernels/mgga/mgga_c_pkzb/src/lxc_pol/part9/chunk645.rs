//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 645/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk645<F: Float>(t2900: F, t2901: F, t302: F, t1125: F, t2099: F, t757: F, t1137: F, t2106: F, t2105: F, t1120: F, t1126: F, t2047: F, t2051: F, t2060: F, t2096: F, t2104: F, t276: F, t2884: F, t2887: F, t2891: F, t2895: F, t2899: F, t735: F) -> (F, F, F, F, F, F) {
    let t2902 = t2900 * t2901;
    let t2903 = t302 * t2902;
    let t2908 = t2099 * t1125;
    let t2909 = t757 * t2908;
    let t2911 = t1137 * t2106;
    let t2912 = t2105 * t2911;
    let t2915 = -t2060 / F::new(108.0) - t2047 - t2051 / F::new(288.0) + t735 * t1120 / F::new(36.0) - t2884 / F::new(288.0) + t2887 * t2891 / F::new(48.0) - t276 * t2895 / F::new(96.0) + F::cast_from(0.42874018118069736972e-3_f64) * t2899 * t2903 - F::cast_from(0.11433071498151929859e-2_f64) * t2096 * t1126 + F::cast_from(0.14291339372689912324e-3_f64) * t2909 - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t2912;
    (t2902, t2903, t2908, t2911, t2912, t2915)
}
