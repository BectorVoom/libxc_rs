//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 774/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk774<F: Float>(t9088: F, t9621: F, t9625: F, t9628: F, t9097: F, t9107: F, t9112: F, t9114: F, t9119: F, t9124: F, t9637: F, t38414: F, t38460: F, t38559: F, t38562: F, t38622: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42559 = 0.1702583995731913576e-4 * t9088;
    let t42560 = 0.23948483403727617128e0 * t9621;
    let t42561 = 0.23948483403727617128e0 * t9625;
    let t42562 = 0.23948483403727617128e0 * t9628;
    let t42563 = 0.5107751987195740728e-4 * t9097;
    let t42567 = 0.5107751987195740728e-4 * t9107;
    let t42568 = 0.1702583995731913576e-4 * t9112;
    let t42569 = 0.1702583995731913576e-4 * t9114;
    let t42570 = 0.638468998399467591e-4 * t9119;
    let t42574 = 0.212822999466489197e-4 * t9124;
    let t42575 = 0.79828278012425390428e-1 * t9637;
    let t42609 = 0.39726959900411316772e-4 * t38414;
    let t42621 = 0.11173207471990682842e-3 * t38460;
    let t42665 = 0.162600798888400151e-2 * t38559;
    let t42666 = 0.162600798888400151e-2 * t38562;
    let t42685 = 0.49658699875514145965e-4 * t38622;
    (t42559, t42560, t42561, t42562, t42563, t42567, t42568, t42569, t42570, t42574, t42575, t42609, t42621, t42665, t42666, t42685)
}
