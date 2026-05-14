//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1187/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1187<F: Float>(t112020: F, t28677: F, t33939: F, t5585: F, t7003: F, t213: F, t231: F, t2726: F, t6819: F, t4113: F, t28652: F, t1091: F, t112015: F, t112016: F, t112018: F, t1196: F, t14770: F, t6045: F, t6833: F, t70452: F, t98539: F, t98544: F, t98545: F, t98563: F, t98593: F) -> (F,) {
    let t112021 = t28677 * t112020;
    let t112023 = t33939 * t5585;
    let t112024 = t7003 * t112023;
    let t112027 = t6819 * t231 * t213 * t2726;
    let t112030 = t4113 * t112023;
    let t112033 = t28652 * t112020;
    let t112046 = 0.13335600218518518519e0 * t98544 * t98545 * t1091 * t14770 - t112015 + 0.37043333940329218109e-2 * t112016 - 0.16111841180489911311e0 * t112018 + 0.16111841180489911311e0 * t112021 + 0.72503285312204600898e0 * t112024 * t112027 - 0.72503285312204600898e0 * t112030 * t112027 - 0.16111841180489911311e0 * t112033 - 0.33339000546296296298e-1 * t98593 * t6833 + 0.12002040196666666667e1 * t98539 * t6045 * t231 * t70452 - 0.60010200983333333334e0 * t98563 * t6045 * t231 * t1196 * t2726;
    (t112046,)
}
