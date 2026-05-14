//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 870/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk870<F: Float>(t2055: F, t5517: F, t72: F, t8094: F, t686: F, t25878: F, t25895: F, t1882: F, t543: F, t7506: F, t7301: F, t27884: F, t7515: F, t25921: F, t26232: F, t26235: F, t26238: F, t26251: F, t26253: F, t26263: F, t26266: F, t26268: F, t26272: F, t7295: F, t8100: F) -> (F, F) {
    let t28760 = t5517 * t2055;
    let t28779 = t8094 * t72;
    let t28780 = t28779 * t686;
    let t28781 = t25878 * t28780;
    let t28783 = t25895 * t28780;
    let t28791 = t7506 * t1882 * t543;
    let t28792 = t7301 * t28791;
    let t28796 = t27884 * t7515;
    let t28799 = -0.72280234901709995518e-2 * t26232 + 0.25702851531048074406e-1 * t28781 - 0.14456046980341999104e-1 * t28783 - 0.14456046980341999104e-1 * t26235 - t26238 + t26251 + 0.9757440539382783019e-2 * t26253 - t26263 - 0.9757440539382783019e-2 * t26266 + 0.4336814094102599731e0 * t25921 * t8100 + 0.4336814094102599731e0 * t7295 * t28792 + 0.12851425765524037203e-1 * t26268 - 0.12851425765524037203e-1 * t28796 + 0.72280234901709995518e-2 * t26272;
    (t28760, t28799)
}
