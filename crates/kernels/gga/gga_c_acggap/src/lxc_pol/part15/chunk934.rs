//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 934/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk934<F: Float>(t34865: F, t34893: F, t34895: F, t34957: F, t34990: F, t35039: F, t35041: F, t35051: F, t35070: F, t35072: F, t35074: F, t35088: F, t35090: F, t35092: F, t35096: F, t35113: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37282 = 0.28582678745379824648e-3 * t34865;
    let t37291 = 0.3361875e0 * t34893;
    let t37292 = 0.3361875e0 * t34895;
    let t37311 = 0.57165357490759649296e-3 * t34957;
    let t37321 = 0.57165357490759649296e-3 * t34990;
    let t37361 = 7.0 / 36.0 * t35039;
    let t37362 = 7.0 / 36.0 * t35041;
    let t37365 = 0.28582678745379824648e-3 * t35051;
    let t37372 = 0.16809375e0 * t35070;
    let t37373 = 0.16809375e0 * t35072;
    let t37374 = 0.1120625e0 * t35074;
    let t37379 = 0.42874018118069736972e-3 * t35088;
    let t37380 = 0.11321313224257494745e-1 * t35090;
    let t37381 = 0.37737710747524982482e-2 * t35092;
    let t37382 = 0.42874018118069736972e-2 * t35096;
    let t37386 = 0.18868855373762491241e-1 * t35113;
    (t37282, t37291, t37292, t37311, t37321, t37361, t37362, t37365, t37372, t37373, t37374, t37379, t37380, t37381, t37382, t37386)
}
