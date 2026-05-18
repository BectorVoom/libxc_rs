//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1406/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1406<F: Float>(t12125: F, t1580: F, t188: F, t189: F, t193: F, t31412: F, t31414: F, t31416: F, t35201: F, t35206: F, t35209: F, t35211: F, t35214: F, t35219: F, t35226: F, t35229: F, t35232: F, t3695: F, t3715: F, t38313: F, t4585: F, t4637: F, t557: F) -> F {
    let t38863 = -t35201 - t35206 - t35209 + t35211 - t35214 - t35219 - t31412 - t31414 - F::new(0.76685851907841499354e0) * t31416 + F::new(0.79445533226334281487e-1) * t557 * t4585 * t3695 + F::new(0.23005755572352449806e1) * t4637 * t3715 + F::new(0.46011511144704899612e1) * t1580 * t12125 + F::new(0.35750489951850426669e0) * t188 * t189 * t38313 * t193 - t35226 - t35229 + t35232;
    t38863
}
