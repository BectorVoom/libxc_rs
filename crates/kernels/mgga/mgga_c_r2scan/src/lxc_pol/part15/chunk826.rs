//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 826/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk826<F: Float>(t1592: F, t1595: F, t6455: F, t6459: F, t6463: F, t6468: F, t6472: F, t6478: F, t6483: F, t6496: F, t6501: F, t6505: F, t8227: F, t8231: F, t8234: F, t8237: F, t8240: F, t8245: F) -> (F,) {
    let t8255 = -0.16463622957338778997e-1 * t8227 - 0.11557628986739024751e0 * t6455 + t8231 - t8234 + 0.13002332610081402845e0 * t1592 * t8237 + 0.2600466522016280569e0 * t8240 * t1595 - t8245 + 0.34672886960217074253e0 * t6459 - 0.57829097596741960692e-3 * t6463 + 0.23115257973478049502e0 * t6468 + 0.11557628986739024751e0 * t6472 - 0.12695991786046386926e-1 * t6478 - 0.38087975358139160778e-1 * t6483 + 0.69345773920434148506e0 * t6496 - 0.13869154784086829701e1 * t6501 - 0.25426783770825854452e1 * t6505;
    (t8255,)
}
