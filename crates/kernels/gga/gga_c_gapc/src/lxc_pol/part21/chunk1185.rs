//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1185/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1185<F: Float>(t11311: F, t11317: F, t1932: F, t11483: F, t628: F, t11489: F, t11316: F, t3064: F, t3954: F, t1030: F, t33303: F, t3123: F) -> (F, F, F, F, F, F) {
    let t34673 = t1932 * t11311 * t11317;
    let t34675 = t628 * t11483;
    let t34676 = t34675 * t11489;
    let t34679 = t11316 * t3064 * t3954;
    let t34681 = t1030 * t33303;
    let t34682 = t34681 * t3123;
    (t34673, t34675, t34676, t34679, t34681, t34682)
}
