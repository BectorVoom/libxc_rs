//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2991/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2991(t11710: f64, t15958: f64, t3091: f64, t1042: f64, t1063: f64, t11672: f64, t11675: f64, t11927: f64, t15615: f64, t15622: f64, t15837: f64, t15938: f64, t15959: f64, t16070: f64, t3117: f64, t3188: f64, t43285: f64, t4786: f64, t53474: f64, t54533: f64, t54537: f64, t54542: f64, t54546: f64, t54550: f64) -> f64 {
    let t54553 = t3091 * t11710 * t15958;
    let t54559 = 0.12862205435420921092e-2_f64 * t11927 * t3117 * t15837 * t4786 + 0.85748036236139473944e-3_f64 * t54533 + 0.25724410870841842183e-2_f64 * t43285 * t15622 - 0.76220476654346199062e-2_f64 * t1063 * t1042 * t54537 * t53474 + 0.64311027177104605458e-3_f64 * t54542 * t16070 + 0.47637797908966374413e-3_f64 * t54546 - 0.45732285992607719436e-2_f64 * t11672 * t15615 + 0.57165357490759649295e-3_f64 * t54550 + 0.57165357490759649295e-3_f64 * t54553 + 0.85748036236139473944e-3_f64 * t11675 * t15959 + 0.25724410870841842183e-2_f64 * t3188 * t15938;
    t54559
}
