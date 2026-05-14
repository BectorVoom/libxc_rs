//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1163/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1163<F: Float>(t34441: F, t10600: F, t1397: F, t1424: F, t2875: F, t544: F, t6540: F, t2299: F, t8070: F, t10609: F, t4781: F, t10241: F, t1323: F, t20827: F, t6717: F, t10314: F, t20441: F, t6716: F) -> (F, F, F, F, F, F, F, F) {
    let t34442 = 0.23005755572352449806e1 * t34441;
    let t34445 = 0.79445533226334281486e-1 * t1397 * t10600 * t1424;
    let t34449 = 0.79445533226334281486e-1 * t544 * t6540 * t2875 * t1424;
    let t34454 = 0.79445533226334281486e-1 * t544 * t2299 * t8070 * t1424;
    let t34457 = t4781 * t10609;
    let t34458 = 0.1533717038156829987e1 * t34457;
    let t34459 = t10241 * t1323;
    let t34462 = 0.13803453343411469884e2 * t20827 * t6717 * t34459;
    let t34465 = 0.18404604457881959845e2 * t6716 * t20441 * t10314;
    (t34442, t34445, t34449, t34454, t34458, t34459, t34462, t34465)
}
