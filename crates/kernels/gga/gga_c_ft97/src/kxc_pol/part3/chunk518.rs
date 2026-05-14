//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 518/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk518<F: Float>(t35: F, t4466: F, t374: F, t1594: F, t4449: F, t938: F, t1711: F, t64: F, t1737: F, t4417: F, t420: F, t419: F, t1742: F, t423: F, t4431: F, t1731: F, t3086: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4467 = t4466 * t35;
    let t4468 = t374 * t4467;
    let t4471 = t1594 * t4449;
    let t4474 = t938 * t938;
    let t4475 = t1711 * t4474;
    let t4476 = t64 * t4475;
    let t4479 = t1737 * t4417;
    let t4480 = t420 * t4479;
    let t4481 = t419 * t4480;
    let t4483 = t1742 * t4417;
    let t4484 = t420 * t4483;
    let t4485 = t419 * t4484;
    let t4487 = t423 * t4431;
    let t4488 = t420 * t4487;
    let t4489 = t419 * t4488;
    let t4491 = -t1731 + 0.42562405586419753086e-2 * t3086 + 0.85124811172839506173e-2 * t4481 - 0.12768721675925925926e-1 * t4485 + 0.6384360837962962963e-2 * t4489;
    (t4467, t4468, t4471, t4474, t4476, t4479, t4480, t4481, t4483, t4484, t4485, t4487, t4488, t4489, t4491)
}
