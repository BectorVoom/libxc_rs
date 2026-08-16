//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1181/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1181(t10506: f64, t95773: f64, t11010: f64, t25391: f64, t26550: f64, t27353: f64, t2828: f64, t39620: f64, t7070: f64, t7071: f64, t7398: f64, t7403: f64, t93104: f64, t93267: f64, t93349: f64, t93351: f64, t95732: f64, t95733: f64, t95740: f64, t95744: f64, t95747: f64, t95762: f64, t95766: f64, t95768: f64) -> f64 {
    let t95774 = t95773 * t10506;
    let t95776 = -t95732 + 0.77108554593144223218e-1_f64 * t95733 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t7398 * t2828 - 0.72280234901709995519e-3_f64 * t95740 - 0.58544643236296698113e-1_f64 * t95744 + 0.68549505033305214441e-2_f64 * t95747 + 0.13010442282307799193e1_f64 * t27353 * t26550 * t39620 - 0.26020884564615598386e1_f64 * t25391 * t26550 * t93267 + 0.78062653693846795158e1_f64 * t93349 * t26550 * t93351 - 0.39512695097613069591e1_f64 * t7403 * t11010 - 0.23132566377943266966e0_f64 * t95762 + 0.15421710918628844643e0_f64 * t95766 + 0.13010442282307799194e0_f64 * t95768 - 0.26020884564615598386e1_f64 * t25391 * t26550 * t93104 - 0.34697458558045176417e-2_f64 * t95774;
    t95776
}
