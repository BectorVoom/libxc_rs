//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1004/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1004<F: Float>(t3180: F, t6711: F, t3134: F, t6538: F, t6188: F, t343: F, t8840: F, t337: F, t2121: F, t2134: F, t6445: F, t6447: F) -> (F, F, F, F, F, F, F) {
    let t9021 = t6711 * t3180 / F::new(48.0);
    let t9023 = t6538 * t3134 / F::new(96.0);
    let t9025 = t6188 * t3134 / F::new(96.0);
    let t9026 = t8840 * t343;
    let t9027 = t337 * t9026;
    let t9028 = t2121 * t9027;
    let t9030 = t2134 * t9028 / F::new(48.0);
    let t9031 = F::new(7.0) / F::new(288.0) * t6445;
    let t9032 = F::new(7.0) / F::new(288.0) * t6447;
    (t9021, t9023, t9025, t9026, t9030, t9031, t9032)
}
