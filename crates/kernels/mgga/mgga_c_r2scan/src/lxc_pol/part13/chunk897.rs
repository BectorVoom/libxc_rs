//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 897/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk897<F: Float>(t1568: F, t8089: F, t7623: F, t1592: F, t1595: F, t6455: F, t6459: F, t6463: F, t6468: F, t6472: F, t6478: F, t6483: F, t6496: F, t6501: F, t6505: F, t8227: F, t8231: F, t8234: F, t8237: F, t8240: F) -> (F, F) {
    let t8243 = t1568 * t8089;
    let t8245 = F::cast_from(0.10975748638225852664e-1_f64) * t7623 * t8243;
    let t8255 = -F::cast_from(0.16463622957338778997e-1_f64) * t8227 - F::cast_from(0.11557628986739024751e0_f64) * t6455 + t8231 - t8234 + F::cast_from(0.13002332610081402845e0_f64) * t1592 * t8237 + F::cast_from(0.2600466522016280569e0_f64) * t8240 * t1595 - t8245 + F::cast_from(0.34672886960217074253e0_f64) * t6459 - F::cast_from(0.57829097596741960692e-3_f64) * t6463 + F::cast_from(0.23115257973478049502e0_f64) * t6468 + F::cast_from(0.11557628986739024751e0_f64) * t6472 - F::cast_from(0.12695991786046386926e-1_f64) * t6478 - F::cast_from(0.38087975358139160778e-1_f64) * t6483 + F::cast_from(0.69345773920434148506e0_f64) * t6496 - F::cast_from(0.13869154784086829701e1_f64) * t6501 - F::cast_from(0.25426783770825854452e1_f64) * t6505;
    (t8243, t8255)
}
