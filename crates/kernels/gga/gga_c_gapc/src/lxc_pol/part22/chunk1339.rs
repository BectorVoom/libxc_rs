//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1339/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1339<F: Float>(t35890: F, t35895: F, t35898: F, t35901: F, t35903: F, t35907: F, t35909: F, t35912: F, t35915: F, t35919: F, t35921: F, t35923: F, t35925: F) -> F {
    let t36204 = -F::new(0.68394856556563412152e-6) * t35890 - F::new(0.61555370900907070936e-5) * t35895 - F::new(0.9785817663350589914e-7) * t35898 + F::new(0.12843885683147649262e-5) * t35901 - F::new(0.46971924784082831588e-3) * t35903 - F::new(0.12843885683147649262e-5) * t35907 + F::new(0.46971924784082831588e-4) * t35909 - F::new(0.12843885683147649262e-5) * t35912 + F::new(0.11957126129729479479e-6) * t35915 + F::new(0.14678726495025884871e-5) * t35919 - F::new(0.32293198289056946716e-4) * t35921 - F::new(0.18788769913633132635e-3) * t35923 - F::new(0.20299047773010240345e-5) * t35925;
    t36204
}
