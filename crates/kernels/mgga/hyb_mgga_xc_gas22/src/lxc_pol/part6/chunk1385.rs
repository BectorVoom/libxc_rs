//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1385/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1385<F: Float>(t25257: F, t3518: F, t3579: F, t1006: F, t21676: F, t2577: F, t29996: F, t29999: F, t30002: F, t30004: F, t30007: F, t30009: F, t30012: F, t30015: F, t30018: F, t30021: F, t30024: F, t30028: F, t30031: F, t30034: F, t30038: F, t4284: F, t4297: F, t7154: F) -> (F, F, F) {
    let t30040 = F::cast_from(0.64327917994770140268e2_f64) * t25257 * t3518;
    let t30041 = t3579 * t3579;
    let t30045 = -F::cast_from(2.0_f64) * t21676 * t4284 + F::cast_from(1.0_f64) * t7154 * t4297 + t29996 + t29999 - t30002 + t30004 + t30007 - t30009 + t30012 + t30015 + t30018 + t30021 - t30024 - t30028 - t30031 - t30034 - t30038 - t30040 - F::cast_from(0.23392894490538584828e1_f64) * t2577 * t30041 * t1006;
    (t30040, t30041, t30045)
}
