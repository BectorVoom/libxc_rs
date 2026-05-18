//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 691/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk691<F: Float>(t1016: F, t9243: F, t3366: F, t6556: F, t3145: F, t4349: F, t3207: F, t1382: F, t12766: F, t1445: F, t597: F, t1645: F, t3137: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12851 = t9243 * t1016;
    let t12853 = F::new(4.0) * t6556 * t3366;
    let t12856 = t1016 * t3145;
    let t12858 = F::new(6.0) * t4349 * t12856;
    let t12862 = t1016 * t3207;
    let t12864 = F::new(2.0) * t1382 * t12862;
    let t12868 = t1445 * t12766;
    let t12870 = F::new(0.11502877786176224903e2) * t597 * t12868;
    let t12871 = t1645 * t3137;
    (t12851, t12853, t12856, t12858, t12862, t12864, t12868, t12870, t12871)
}
