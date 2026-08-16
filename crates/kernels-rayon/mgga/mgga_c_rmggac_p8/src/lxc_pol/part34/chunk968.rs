//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 968/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk968(t74562: f64, t74574: f64, t74577: f64, t74579: f64, t74581: f64, t74584: f64, t74590: f64, t15616: f64, t2106: f64, t2145: f64, t14683: f64, t8577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77105 = 0.29085809927086856922e-4_f64 * t74562;
    let t77107 = 0.23268647941669485538e-4_f64 * t74574;
    let t77108 = 0.1276937996798935182e-4_f64 * t74577;
    let t77109 = 0.85129199786595678799e-5_f64 * t74579;
    let t77110 = 0.85129199786595678799e-5_f64 * t74581;
    let t77111 = 0.85129199786595678799e-5_f64 * t74584;
    let t77113 = 0.2627895913935205078e-5_f64 * t74590;
    let t77116 = t2145 * t15616 * t2106;
    let t77117 = 0.90915538847484472429e-2_f64 * t77116;
    let t77118 = t8577 * t14683;
    (t77105, t77107, t77108, t77109, t77110, t77111, t77113, t77117, t77118)
}
