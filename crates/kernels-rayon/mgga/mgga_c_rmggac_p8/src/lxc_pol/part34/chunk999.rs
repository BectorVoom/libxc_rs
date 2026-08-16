//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 999/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk999(t69586: f64, t71380: f64, t75137: f64, t75139: f64, t75145: f64, t75149: f64, t75152: f64, t75157: f64, t570: f64, t71704: f64, t75163: f64, t1356: f64, t14435: f64, t5928: f64, t69599: f64, t71369: f64, t71373: f64, t71376: f64, t75134: f64, t75143: f64) -> (f64, f64) {
    let t77476 = 0.20496175532535769483e-3_f64 * t69586;
    let t77477 = 0.15243824895787514157e-3_f64 * t71380;
    let t77480 = 0.19709219354514038085e-5_f64 * t75137;
    let t77481 = 0.638468998399467591e-4_f64 * t75139;
    let t77484 = 0.1276937996798935182e-4_f64 * t75145;
    let t77485 = 0.1276937996798935182e-4_f64 * t75149;
    let t77486 = 0.1276937996798935182e-4_f64 * t75152;
    let t77487 = 0.16360768083986357019e-4_f64 * t75157;
    let t77488 = t71704 * t570;
    let t77491 = 0.44903406381989282115e-1_f64 * t75163;
    let t77492 = -t71369 - t77476 + t71373 - t71376 - t77477 + 0.36357262408858571154e-4_f64 * t69599 - 0.17519306092901367187e-5_f64 * t75134 - t77480 - t77481 + t75143 + 0.39914139006212695214e-1_f64 * t5928 * t14435 - t77484 + t77485 + t77486 + t77487 + 0.39914139006212695214e-1_f64 * t1356 * t77488 - t77491;
    (t77488, t77492)
}
