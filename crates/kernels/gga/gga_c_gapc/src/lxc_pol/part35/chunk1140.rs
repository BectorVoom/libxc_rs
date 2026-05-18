//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1140/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1140<F: Float>(t11522: F, t15805: F, t9799: F, t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34125: F, t34127: F, t34132: F) -> F {
    let t34135 = t15805 * t11522 * t9799;
    let t34137 = F::new(0.2318836277704281739e-4) * t34104 + F::new(0.56360603971979070047e-7) * t34108 + F::new(0.34752370105806885418e-3) * t34111 - F::new(0.24581606547037760418e-7) * t34114 + F::new(0.12290803273518880209e-8) * t34117 - F::new(0.35170937063461460536e-8) * t34119 - F::new(0.35170937063461460536e-8) * t34121 + F::new(0.4797801045921060808e-7) * t34125 + F::new(0.17089546493091976008e-5) * t34127 - F::new(0.12290803273518880209e-8) * t34132 + F::new(0.12650553385416666667e-5) * t34135;
    t34137
}
