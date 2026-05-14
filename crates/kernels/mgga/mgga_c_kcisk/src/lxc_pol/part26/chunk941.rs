//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 941/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk941<F: Float>(t13048: F, t7754: F, t3634: F, t7786: F, t7779: F, t827: F, t7782: F, t7776: F, t1186: F, t25313: F, t26: F, t25446: F, t3661: F, t12935: F, t12999: F, t13000: F, t19102: F, t19540: F, t19543: F, t19545: F) -> (F, F, F, F, F, F, F, F) {
    let t25685 = 2.0 * t13048 * t7754;
    let t25687 = 1.0 * t3634 * t7786;
    let t25696 = t827 * t7779;
    let t25699 = t827 * t7782;
    let t25701 = t827 * t7776;
    let t25703 = t1186 * t25313;
    let t25704 = t26 * t25703;
    let t25710 = t3661 * t25446;
    let t25711 = t26 * t25710;
    let t25713 = 0.54771111111111111111e-1 * t25699 + 0.18257037037037037037e-1 * t25701 - 0.82156666666666666667e-1 * t25704 - 0.91285185185185185187e-1 * t12935 + 0.13287407407407407407e0 * t19102 - t19540 - t12999 - t13000 - 0.18257037037037037037e0 * t19543 + 0.21908444444444444444e0 * t19545 + 0.16431333333333333333e0 * t25711;
    (t25685, t25687, t25696, t25699, t25701, t25704, t25711, t25713)
}
