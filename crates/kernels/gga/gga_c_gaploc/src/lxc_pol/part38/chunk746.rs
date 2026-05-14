//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 746/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk746<F: Float>(t10295: F, t27214: F, t3073: F, t3459: F, t5559: F, t35781: F, t977: F, t3418: F, t8042: F, t11288: F, t2497: F, t10800: F, t11127: F, t44670: F, t44671: F, t44674: F, t44676: F, t44678: F, t44679: F, t44681: F, t44684: F, t44687: F, t44689: F, t44692: F, t8862: F) -> (F, F, F, F) {
    let t44694 = 12.0 * t27214 * t10295;
    let t44697 = 12.0 * t5559 * t3459 * t3073;
    let t44702 = t35781 * t977;
    let t44704 = 2.0 * t8042 * t3418;
    let t44705 = t11288 * t2497;
    let t44706 = -2.0 * t10800 * t3073 + 4.0 * t11127 * t8862 - t44670 + t44671 + t44674 - t44676 + t44678 + t44679 - t44681 + t44684 - t44687 + t44689 - t44692 + t44694 - t44697 - t44702 + t44704 + t44705;
    (t44694, t44704, t44705, t44706)
}
