//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1065/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1065<F: Float>(t190: F, t2660: F, t286: F, t33235: F, t442: F, t8139: F, t28924: F, t3784: F, t33209: F, t33212: F, t33214: F, t33217: F, t33221: F, t33226: F, t33228: F, t33230: F, t33232: F) -> F {
    let t33240 = t2660 * t33235 * t8139 * t190 * t286 * t442;
    let t33242 = t3784 * t28924;
    let t33244 = F::cast_from(0.67528199161846004232e-6_f64) * t33209 + F::cast_from(0.18115908419564701086e-6_f64) * t33212 - F::cast_from(0.10129229874276900635e-5_f64) * t33214 + F::cast_from(0.90579542097823505428e-7_f64) * t33217 + F::cast_from(0.82779637083844259127e-6_f64) * t33221 + F::cast_from(0.59920486569434427612e-7_f64) * t33226 - F::cast_from(0.12650553385416666667e-5_f64) * t33228 + F::cast_from(0.9275345110817126956e-4_f64) * t33230 + F::cast_from(0.77294542590142724635e-6_f64) * t33232 - F::cast_from(0.12187980608940473897e-4_f64) * t33240 - F::cast_from(0.33147827249531850014e-7_f64) * t33242;
    t33244
}
