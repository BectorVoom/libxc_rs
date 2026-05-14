//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1391/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1391<F: Float>(t114455: F, t114464: F, t115704: F, t115705: F, t115708: F, t119427: F, t119430: F, t119438: F, t20: F, t2734: F, t2736: F, t2740: F, t27906: F, t32339: F, t33771: F, t33794: F, t34940: F, t34950: F, t394: F, t79: F, t84770: F, t9511: F) -> (F,) {
    let t120576 = -t115704 + t115705 + 0.69644166666666666664e-2 * t119427 - 0.23214722222222222222e-2 * t119430 - 0.52083333333333333333e-2 * t2734 * t27906 * t394 * t20 * t2740 - 0.52083333333333333333e-2 * t9511 * t34940 * t2740 - 0.52083333333333333333e-2 * t84770 * t79 * t2736 * t2740 - 0.23214722222222222222e-2 * t119438 + t115708 + 0.30952962962962962962e-2 * t114455 - 0.77382407407407407407e-3 * t114464 + 0.34722222222222222223e-2 * t33794 * t33771 - 0.92592592592592592592e-2 * t32339 * t34950;
    (t120576,)
}
