//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 350/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk350<F: Float>(t1445: F, t2846: F, t1000: F, t1004: F, t1008: F, t1013: F, t1456: F, t1580: F, t1599: F, t1641: F, t193: F, t2362: F, t2369: F, t2390: F, t2411: F, t2804: F, t2807: F, t2810: F, t2816: F, t2819: F, t2823: F, t2828: F, t2834: F, t2843: F, t541: F, t557: F, t574: F, t597: F) -> F {
    let t2847 = t1445 * t2846;
    let t2850 = F::new(0.30674340763136599741e1) * t597 * t2804 - F::new(0.23833659967900284446e0) * t557 * t2807 - F::new(0.30674340763136599741e1) * t574 * t2810 + F::new(0.23833659967900284446e0) * t1000 * t541 + F::new(0.23005755572352449806e1) * t597 * t2816 + F::new(0.35750489951850426669e0) * t2819 * t193 + F::new(0.35750489951850426669e0) * t2823 * t193 - F::new(0.35750489951850426669e0) * t1599 * t1004 - F::new(0.35750489951850426669e0) * t557 * t2828 - F::new(0.23005755572352449806e1) * t1641 * t1008 - F::new(0.23005755572352449806e1) * t574 * t2834 + F::new(0.23005755572352449806e1) * t1580 * t1013 + F::new(0.25561950635947166451e0) * t2362 - F::new(0.29792074959875355558e-1) * t2369 - F::new(0.59584149919750711116e-1) * t2390 + F::new(0.29792074959875355558e-1) * t2411 + F::new(0.35750489951850426669e0) * t1456 * t2843 - F::new(0.46011511144704899612e1) * t574 * t2847;
    t2850
}
