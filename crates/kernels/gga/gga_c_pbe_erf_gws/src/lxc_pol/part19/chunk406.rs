//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 406/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk406<F: Float>(t156: F, t472: F, t1447: F, t532: F, t535: F, t159: F, t285: F, t545: F, t762: F, t147: F, t39: F, t169: F, t274: F, t301: F, t366: F, t5: F, t784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1448 = t156 * t472;
    let t1449 = t1447 * t1448;
    let t1450 = 0.10843580882781524214e-1 * t1449;
    let t1457 = t532 * t535;
    let t1459 = t1457 * t159 * t285;
    let t1463 = 0.58113483035773838734e-3 * t762 * t545 * t285;
    let t1464 = t39 * t147;
    let t1467 = 0.13559812708347229038e-2 * t1464 * t159 * t285;
    let t1471 = 0.19816831758676854261e0 * t169 * t366 * t274 * t301;
    let t1472 = t5 * t784;
    (t1448, t1450, t1457, t1459, t1463, t1464, t1467, t1471, t1472)
}
