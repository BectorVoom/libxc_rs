//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1042/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1042<F: Float>(t93317: F, t95761: F, t26492: F, t686: F, t72: F, t25387: F, t93281: F, t2453: F, t26496: F, t10506: F, t11010: F, t25391: F, t26550: F, t27353: F, t2828: F, t39620: F, t7070: F, t7071: F, t7398: F, t7403: F, t93104: F, t93267: F, t93349: F, t93351: F, t95732: F, t95733: F, t95740: F, t95744: F, t95747: F) -> (F, F) {
    let t95762 = t93317 * t95761;
    let t95765 = t26492 * t72 * t686;
    let t95766 = t25387 * t95765;
    let t95768 = t93281 * t95761;
    let t95773 = t2453 * t26496;
    let t95774 = t95773 * t10506;
    let t95776 = -t95732 + 0.77108554593144223218e-1 * t95733 + 0.26020884564615598386e1 * t7070 * t7071 * t7398 * t2828 - 0.72280234901709995519e-3 * t95740 - 0.58544643236296698113e-1 * t95744 + 0.68549505033305214441e-2 * t95747 + 0.13010442282307799193e1 * t27353 * t26550 * t39620 - 0.26020884564615598386e1 * t25391 * t26550 * t93267 + 0.78062653693846795158e1 * t93349 * t26550 * t93351 - 0.39512695097613069591e1 * t7403 * t11010 - 0.23132566377943266966e0 * t95762 + 0.15421710918628844643e0 * t95766 + 0.13010442282307799194e0 * t95768 - 0.26020884564615598386e1 * t25391 * t26550 * t93104 - 0.34697458558045176417e-2 * t95774;
    (t95765, t95776)
}
