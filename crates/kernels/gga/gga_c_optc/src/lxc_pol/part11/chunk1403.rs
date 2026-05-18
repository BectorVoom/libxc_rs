//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1403/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1403<F: Float>(t17423: F, t4144: F, t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t52389: F, t52391: F, t52393: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F) -> (F, F) {
    let t59088 = F::new(4.0) * t4144 * t17423;
    let t59103 = -F::new(0.18257037037037037037e0) * t44193 + F::new(0.10954222222222222222e1) * t44198 - F::new(0.5314962962962962963e0) * t43414 + F::new(0.12401580246913580247e1) * t33724 + F::new(0.97370864197530864199e0) * t33730 + F::new(0.23917333333333333333e1) * t58348 + F::new(0.98587999999999999999e0) * t58352 - F::new(0.21908444444444444444e0) * t58356 - F::new(0.295764e1) * t58360 + F::new(0.65725333333333333332e0) * t58363 - F::new(0.10954222222222222222e0) * t58367 + F::new(0.39862222222222222223e0) * t52389 + F::new(0.23917333333333333333e1) * t52391 + F::new(0.44291358024691358024e0) * t52393;
    (t59088, t59103)
}
