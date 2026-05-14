//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 961/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk961<F: Float>(t3587: F, t5730: F, t19104: F, t2089: F, t2877: F, t16391: F, t5745: F, t12969: F, t2083: F, t3559: F, t1180: F, t19144: F, t12967: F, t12999: F, t13000: F, t19111: F, t19192: F, t19207: F) -> (F, F, F, F, F, F) {
    let t19528 = t5730 * t3587;
    let t19540 = 0.39862222222222222222e0 * t19104;
    let t19543 = t2877 * t2089;
    let t19545 = t16391 * t5745;
    let t19548 = t12969 * t2083;
    let t19549 = t19548 * t3559;
    let t19551 = t1180 * t19144;
    let t19554 = -t19540 - 0.33218518518518518518e0 * t19111 - 0.10954222222222222222e0 * t12967 - t12999 - t13000 - 0.91285185185185185185e-1 * t19543 + 0.71202444444444444443e0 * t19545 + 0.142419375e1 * t19207 - 0.76790625e-1 * t19549 + 0.3071625e0 * t19551 + 0.1898925e1 * t19192;
    (t19528, t19543, t19545, t19549, t19551, t19554)
}
