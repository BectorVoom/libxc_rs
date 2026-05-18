//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 995/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk995<F: Float>(t13746: F, t13748: F, t26198: F, t30306: F, t30353: F, t30355: F, t30357: F, t30360: F, t30363: F, t30366: F, t30369: F, t30372: F, t30375: F, t30377: F) -> F {
    let t30434 = -F::new(0.301925e0) * t30306 + F::new(0.33114e0) * t26198 + F::new(0.19419375e1) * t30353 - t13746 - F::new(0.3883875e1) * t30355 + F::new(0.247573125e0) * t30357 + F::new(0.99342e0) * t30360 - F::new(0.16557e0) * t30363 - F::new(0.73586666666666666666e-1) * t30366 - F::new(0.16557e0) * t30369 + F::new(0.33114e0) * t30372 - F::new(0.99342e0) * t30375 + F::new(0.16504875e0) * t30377 - t13748;
    t30434
}
