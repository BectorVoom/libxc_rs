//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 851/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk851<F: Float>(t299: F, t22487: F, t1113: F, t202: F, t237: F, t1100: F, t1416: F, t226: F, t6762: F, t287: F, t4092: F, t2393: F, t4939: F) -> (F, F, F, F, F, F) {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t22488 = piecewise3::<F>(t300, F::new(0.0), t22487);
    let t27703 = t202 * t1113;
    let t27704 = t27703 * t237;
    let t27711 = t1100 * t1416;
    let t27733 = t6762 * t226;
    let t28676 = t4092 * t287;
    let t30651 = t2393 * t4939;
    (t22488, t27704, t27711, t27733, t28676, t30651)
}
