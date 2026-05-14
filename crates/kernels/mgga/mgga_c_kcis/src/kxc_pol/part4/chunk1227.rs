//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1227/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1227<F: Float>(t17738: F, t17962: F, t17988: F, t18008: F, t18034: F, t18054: F, t18237: F, t18263: F, t2118: F, t4479: F, t1636: F, t6256: F, t17331: F, t17335: F, t17337: F, t17339: F, t17342: F, t17344: F, t17347: F, t17350: F, t17353: F, t17355: F, t17358: F, t17360: F, t17362: F, t17364: F, t17366: F, t17368: F, t17371: F, t17374: F) -> (F, F, F, F) {
    let t18266 = t17738 + t17962 + t17988 + t18008 + t18034 + t18054 + t18237 + t18263;
    let t18268 = t2118 * t4479;
    let t18271 = t6256 * t1636;
    let t18292 = -0.101171875e-1 * t17331 - 0.53958333333333333333e-1 * t17335 + 0.625e-1 * t17337 - 0.33333333333333333334e0 * t17339 - 0.44965277777777777777e-2 * t17342 - 0.1875e0 * t17344 - 0.41666666666666666666e-1 * t17347 + 0.44965277777777777777e-2 * t17350 + 0.10791666666666666667e0 * t17353 - 0.9375e-1 * t17355 + 0.375e0 * t17358 - 0.9375e-1 * t17360 + 0.53958333333333333333e-1 * t17362 - 0.125e0 * t17364 + 0.26979166666666666666e-1 * t17366 - 0.44965277777777777777e-2 * t17368 + 0.20833333333333333333e-1 * t17371 - 0.4046875e-1 * t17374;
    (t18266, t18268, t18271, t18292)
}
