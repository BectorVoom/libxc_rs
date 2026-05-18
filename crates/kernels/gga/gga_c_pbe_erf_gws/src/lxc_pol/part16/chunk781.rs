//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 781/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk781<F: Float>(t274: F, t413: F, t39: F, t745: F, t1452: F, t532: F, t1553: F, t2718: F, t2704: F, t502: F, t1509: F, t486: F) -> (F, F, F, F, F, F, F) {
    let t5732 = F::new(0.12798016258123051272e1) * t413 * t274;
    let t5733 = t39 * t745;
    let t5735 = t532 * t1452;
    let t5749 = F::new(0.65290666666666666667e0) * t1553 * t2718;
    let t5751 = F::new(0.76172444444444444444e0) * t502 * t2704;
    let t5753 = F::new(0.12991222222222222222e1) * t1509 * t2718;
    let t5755 = F::new(0.15156425925925925926e1) * t486 * t2704;
    (t5732, t5733, t5735, t5749, t5751, t5753, t5755)
}
