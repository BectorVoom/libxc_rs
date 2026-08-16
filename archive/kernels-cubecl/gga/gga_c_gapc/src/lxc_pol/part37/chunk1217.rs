//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1217/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1217<F: Float>(t11302: F, t19844: F, t5974: F, t1743: F, t33148: F, t19644: F, t11356: F, t9071: F, t9256: F, t11604: F, t26836: F, t11468: F, t3065: F) -> (F, F, F, F, F, F) {
    let t34315 = t19844 * t11302 * t5974;
    let t34317 = t1743 * t33148;
    let t34318 = t34317 * t19644;
    let t34321 = t9071 * t11356 * t9256;
    let t34323 = t11604 * t26836;
    let t34325 = t11468 * t3065;
    (t34315, t34317, t34318, t34321, t34323, t34325)
}
