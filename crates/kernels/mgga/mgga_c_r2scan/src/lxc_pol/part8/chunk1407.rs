//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1407/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1407<F: Float>(t2207: F, t785: F, t788: F, t9880: F, t32871: F, t538: F, t6155: F, t20916: F, t20921: F, t25715: F, t25726: F, t25729: F, t25740: F, t25742: F, t30094: F, t30098: F, t30100: F, t30113: F, t30121: F) -> (F,) {
    let t34115 = t2207 * t785 * t788 * t9880;
    let t34125 = t6155 * t538 * t32871;
    let t34129 = -0.17465477326173296717e-1 * t34115 - 0.52690178912667028302e0 * t30094 + 0.86743646395112941038e-3 * t25715 + 0.29634521323209802194e0 * t30098 - 0.98781737744032673978e-1 * t30100 - 0.77115101645255404583e-4 * t25726 - t25729 - t25740 - 0.1047298617893752044e1 * t25742 + 0.1047928639570397803e0 * t30113 - 0.16463622957338778996e-1 * t34125 - 0.19776387377308997907e1 * t20916 + t20921 + 0.38087975358139160776e-1 * t30121;
    (t34129,)
}
