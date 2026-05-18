//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 954/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk954<F: Float>(t10556: F, t544: F, t2392: F, t2482: F, t2890: F, t9267: F, t2299: F, t2875: F, t1424: F, t4130: F, t986: F, t9272: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10557 = t544 * t10556;
    let t10559 = F::new(0.42900587942220512003e1) * t10557 * t2392;
    let t10597 = t2890 * t2482;
    let t10598 = t9267 * t10597;
    let t10599 = F::new(0.9585731488480187419e0) * t10598;
    let t10600 = t2299 * t2875;
    let t10601 = t544 * t10600;
    let t10603 = F::new(0.39722766613167140743e-1) * t10601 * t1424;
    let t10608 = t4130 * t986;
    let t10609 = t10608 * t2482;
    let t10610 = t9272 * t10609;
    (t10557, t10559, t10597, t10599, t10600, t10601, t10603, t10608, t10609, t10610)
}
