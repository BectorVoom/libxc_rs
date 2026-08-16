//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 859/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk859(t1377: f64, t2091: f64, t3886: f64, t794: f64, t8611: f64, t6897: f64, t31153: f64, t31160: f64, t31177: f64, t22674: f64, t8621: f64, t2085: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31549 = t1377 * t2091;
    let t31558 = t3886 * t2091;
    let t31569 = t794 * t8611;
    let t31570 = t6897 * t31569;
    let t31571 = 0.41123351671205660912e-2_f64 * t31570;
    let t31576 = 0.11304371706359309439e-1_f64 * t31153;
    let t31578 = 0.26915170729426927235e-3_f64 * t31160;
    let t31582 = 7.0_f64 / 1152.0_f64 * t31177;
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31596 = 0.41123351671205660912e-2_f64 * t31595;
    let t31611 = t214 * t2085;
    (t31549, t31558, t31569, t31571, t31576, t31578, t31582, t31594, t31596, t31611)
}
