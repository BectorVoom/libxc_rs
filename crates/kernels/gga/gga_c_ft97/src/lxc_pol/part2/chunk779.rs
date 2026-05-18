//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 779/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk779<F: Float>(t11171: F, t11169: F, t11204: F, t528: F, t8690: F, t929: F, t2007: F, t3056: F, t11180: F, t11186: F, t11189: F, t11195: F, t11198: F, t1595: F, t1655: F, t3359: F, t383: F) -> (F, F, F, F) {
    let t12216 = F::new(0.19257444444444444444e0) * t11171;
    let t12217 = F::new(0.6419148148148148148e-1) * t11169;
    let t12223 = t528 * t11204;
    let t12225 = t8690 * t929;
    let t12228 = t2007 * t3056;
    let t12233 = -t12216 + t12217 - F::new(0.9628722222222222222e-1) * t11189 - F::new(0.1604787037037037037e0) * t11180 - F::new(0.38514888888888888888e0) * t11186 + F::new(0.28886166666666666666e0) * t11198 + F::new(0.11554466666666666666e1) * t11195 + F::new(0.234754e0) * t12223 + F::new(0.1760655e0) * t12225 * t1595 - F::new(0.234754e0) * t12228 * t383 - F::new(0.117377e0) * t3359 * t1655;
    (t12223, t12225, t12228, t12233)
}
