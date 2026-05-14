//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 795/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk795<F: Float>(t2457: F, t26276: F, t25944: F, t25950: F, t7515: F, t213: F, t7506: F, t2470: F, t7514: F, t7284: F, t25878: F, t26234: F, t1445: F, t7492: F, t689: F, t1385: F, t2097: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26277 = t26276 * t2457;
    let t26279 = 0.17135234354032049604e-2 * t25944 * t26277;
    let t26280 = t25950 * t7515;
    let t26282 = t213 * t7506;
    let t26292 = t7514 * t2470;
    let t26294 = 0.96373646535613327357e-2 * t7284 * t26292;
    let t26295 = t25878 * t26234;
    let t26301 = t7492 * t1445;
    let t26302 = t689 * t26301;
    let t26304 = t1385 * t2097;
    (t26277, t26279, t26280, t26282, t26292, t26294, t26295, t26302, t26304)
}
