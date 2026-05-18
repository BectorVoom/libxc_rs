//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1126/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1126<F: Float>(t2471: F, t7018: F, t25331: F, t7058: F, t25309: F, t7063: F, t7060: F, t25296: F, t7064: F, t2435: F, t7015: F, t251: F, t786: F) -> (F, F, F, F, F, F, F) {
    let t25362 = F::new(0.13009920719177044025e-1) * t7018 * t2471;
    let t25364 = F::new(0.96373646535613327357e-2) * t7058 * t25331;
    let t25365 = t7063 * t25309;
    let t25366 = t25365 * t7060;
    let t25368 = t7064 * t25296;
    let t25371 = F::new(0.73171657588172351096e-2) * t2435 * t7015;
    let t25372 = t786 * t251;
    (t25362, t25364, t25365, t25366, t25368, t25371, t25372)
}
