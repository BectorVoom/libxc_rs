//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1383/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1383<F: Float>(t33940: F, t9850: F, t1591: F, t32440: F, t6204: F, t8403: F, t109626: F, t109793: F, t109797: F, t115085: F, t115541: F, t115550: F, t115555: F, t115566: F, t115592: F, t115726: F, t119169: F, t119186: F, t27721: F, t32439: F, t33784: F, t9535: F, t9539: F) -> (F, F) {
    let t120369 = t9850 * t33940;
    let t120376 = t6204 * t32440 * t8403 * t1591;
    let t120388 = -0.34722222222222222223e-2 * t120369 * t9539 - 0.120625e-1 * t115085 * t33784 - 0.20104166666666666667e-2 * t32439 * t120376 - 0.23280625e-2 * t115592 * t9535 * t33784 + t115541 + 0.77382407407407407407e-3 * t119169 + 0.46296296296296296296e-2 * t109626 * t115726 * t27721 + t115550 - 0.30864197530864197531e-2 * t109793 - t109797 + 0.46429444444444444444e-2 * t119186 - t115555 - t115566;
    (t120376, t120388)
}
