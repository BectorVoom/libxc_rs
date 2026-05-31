//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1211/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1211<F: Float>(t4631: F, t4635: F, t2924: F, t11404: F, t11548: F, t15400: F, t1622: F, t19046: F, t19079: F, t19130: F, t19132: F, t19173: F, t19227: F, t19247: F, t2938: F, t311: F, t4647: F, t4670: F, t6158: F, t6174: F, t6177: F, t946: F, t955: F) -> (F, F) {
    let t19250 = t4635 * t4631;
    let t19252 = F::cast_from(0.32163958997385070134e2_f64) * t2924 * t19250;
    let t19253 = t19079 - t19130 - t19132 + F::cast_from(1.0_f64) * t19173 * t955 + F::cast_from(2.0_f64) * t15400 * t1622 + F::cast_from(2.0_f64) * t4647 * t4670 - F::cast_from(2.0_f64) * t11548 * t6158 + F::cast_from(1.0_f64) * t2938 * t6174 + F::cast_from(1.0_f64) * t946 * t19227 + F::cast_from(0.32163958997385070134e2_f64) * t11404 * t6177 - F::cast_from(0.19751673498613801407e-1_f64) * t19046 - F::cast_from(0.310907e-1_f64) * t19247 * t311 - t19252;
    (t19252, t19253)
}
