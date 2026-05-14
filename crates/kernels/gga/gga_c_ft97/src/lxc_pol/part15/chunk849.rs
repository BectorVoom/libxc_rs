//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 849/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk849<F: Float>(t21249: F, t694: F, t21237: F, t25: F, t18132: F, t4952: F, t6: F, t1127: F, t5014: F, t20489: F, t2360: F, t2347: F, t21204: F, t701: F, t9483: F, t173: F, t21186: F) -> (F, F, F, F, F, F, F, F, F) {
    let t79489 = t694 * t21249;
    let t79559 = t694 * t21237;
    let t79593 = t21237 * t25;
    let t79622 = t18132 * t6 * t4952;
    let t79629 = t5014 * t1127;
    let t79697 = t2360 * t20489;
    let t79702 = t2347 * t20489;
    let t79714 = t701 * t9483 * t21204;
    let t79757 = t701 * t173 * t21186;
    (t79489, t79559, t79593, t79622, t79629, t79697, t79702, t79714, t79757)
}
