//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1301/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1301<F: Float>(t1947: F, t200: F, t517: F, t8379: F, t8394: F, t144: F, t640: F, t2941: F, t3954: F, t3949: F, t8459: F, t3635: F, t8521: F) -> (F, F, F, F) {
    let t35606 = t8379 * t517 * t8394 * t200 * t1947;
    let t35608 = t640 * t144;
    let t35610 = t2941 * t35608 * t3954;
    let t35613 = t8459 * t35608 * t3949;
    let t35615 = t8521 * t3635;
    (t35606, t35610, t35613, t35615)
}
