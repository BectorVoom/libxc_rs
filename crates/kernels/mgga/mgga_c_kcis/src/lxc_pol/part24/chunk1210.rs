//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1210/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1210<F: Float>(t27761: F, t28302: F, t27741: F, t26657: F, t29225: F, t29228: F, t29238: F, t91769: F, t91772: F, t91773: F, t91776: F, t91777: F, t91778: F, t95270: F, t95271: F, t95272: F, t95273: F, t95274: F, t95276: F) -> (F, F, F) {
    let t97607 = t27761 / F::new(8.0);
    let t97608 = t28302 / F::new(8.0);
    let t99798 = F::new(4.0) * t27741;
    let t99799 = t95270 - t91769 + t91772 + t29238 + t91773 + t95271 - t91776 - t95272 + t95273 + t91777 + t95274 + t99798 - t91778 + t26657 - t29225 - t95276 - t29228;
    (t97607, t97608, t99799)
}
