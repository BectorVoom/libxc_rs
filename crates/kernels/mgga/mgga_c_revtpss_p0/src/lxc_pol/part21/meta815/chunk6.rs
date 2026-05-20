//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2991/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2991<F: Float>(t11710: F, t15958: F, t3091: F, t1042: F, t1063: F, t11672: F, t11675: F, t11927: F, t15615: F, t15622: F, t15837: F, t15938: F, t15959: F, t16070: F, t3117: F, t3188: F, t43285: F, t4786: F, t53474: F, t54533: F, t54537: F, t54542: F, t54546: F, t54550: F) -> F {
    let t54553 = t3091 * t11710 * t15958;
    let t54559 = F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t15837 * t4786 + F::cast_from(0.85748036236139473944e-3_f64) * t54533 + F::cast_from(0.25724410870841842183e-2_f64) * t43285 * t15622 - F::cast_from(0.76220476654346199062e-2_f64) * t1063 * t1042 * t54537 * t53474 + F::cast_from(0.64311027177104605458e-3_f64) * t54542 * t16070 + F::cast_from(0.47637797908966374413e-3_f64) * t54546 - F::cast_from(0.45732285992607719436e-2_f64) * t11672 * t15615 + F::cast_from(0.57165357490759649295e-3_f64) * t54550 + F::cast_from(0.57165357490759649295e-3_f64) * t54553 + F::cast_from(0.85748036236139473944e-3_f64) * t11675 * t15959 + F::cast_from(0.25724410870841842183e-2_f64) * t3188 * t15938;
    t54559
}
