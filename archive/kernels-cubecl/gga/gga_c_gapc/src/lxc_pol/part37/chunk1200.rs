//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1200/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1200<F: Float>(t30153: F, t30158: F, t34041: F, t28427: F, t3784: F, t1089: F, t33304: F, t3322: F, t33494: F, t3330: F, t33312: F, t11808: F, t30187: F) -> (F, F, F, F, F, F) {
    let t34046 = t30153 * t34041 * t30158;
    let t34048 = t3784 * t28427;
    let t34050 = t33304 * t1089;
    let t34052 = t33494 * t3322;
    let t34054 = t33312 * t3330;
    let t34056 = t11808 * t30187;
    (t34046, t34048, t34050, t34052, t34054, t34056)
}
