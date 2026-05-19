//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 902/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk902<F: Float>(t2873: F, t978: F, t10: F, t10051: F, t10054: F, t10065: F, t10069: F, t10072: F, t10075: F, t10078: F, t10081: F, t496: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5764: F, t5776: F, t8126: F, t8137: F, t8139: F, t8142: F) -> F {
    let t10085 = t978 * t2873;
    let t10089 = -t5749 - t5751 + t5753 - t5755 - t5759 - t10051 + t10054 - t496 * t10065 / F::new(2.0) - t10069 / F::new(2.0) + t10072 / F::new(6.0) - F::new(0.293808e1) * t10075 + F::new(0.73452e0) * t10078 - F::cast_from(0.48968000000000000001e0_f64) * t5764 - F::new(6.0) * t496 * t10 * t10081 + F::new(3.0) * t496 * t10 * t10085 - t5776 - t8126 - t8137 - t8139 + t8142;
    t10089
}
