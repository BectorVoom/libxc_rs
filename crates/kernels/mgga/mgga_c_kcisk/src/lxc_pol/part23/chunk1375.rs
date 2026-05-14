//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1375/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1375<F: Float>(t1339: F, t32207: F, t33604: F, t32211: F, t3759: F, t110347: F, t110351: F, t110492: F, t113997: F, t114260: F, t114264: F, t114268: F, t114271: F, t114273: F, t114276: F, t114280: F, t32019: F, t32096: F, t33588: F, t9449: F, t9809: F) -> (F, F, F) {
    let t114293 = t1339 * t33604 * t32207;
    let t114296 = t3759 * t33604 * t32211;
    let t114298 = 0.11054629629629629629e-2 * t114260 + 0.18518518518518518519e-1 * t113997 * t9449 - 0.23148148148148148148e-2 * t114264 - 0.16581944444444444444e-2 * t114268 - t114271 - 0.33163888888888888888e-2 * t114273 + 0.88437037037037037034e-2 * t114276 + 0.49745833333333333332e-2 * t114280 + 0.10416666666666666667e-1 * t110347 * t9809 + 0.20833333333333333334e-1 * t110351 * t9809 + 0.20833333333333333334e-1 * t32096 * t33588 + 0.10416666666666666667e-1 * t110492 * t9809 + 0.20833333333333333334e-1 * t32019 * t33588 + 0.16581944444444444444e-2 * t114293 + 0.27636574074074074073e-2 * t114296;
    (t114293, t114296, t114298)
}
