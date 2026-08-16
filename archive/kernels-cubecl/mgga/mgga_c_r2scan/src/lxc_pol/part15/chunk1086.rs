//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1086/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1086<F: Float>(t38175: F, t10841: F, t1607: F, t2228: F, t505: F, t539: F, t10856: F, t6245: F, t252: F, t3320: F, t6262: F, t783: F) -> (F, F, F, F, F, F) {
    let t38176 = F::cast_from(0.174549769648958674e0_f64) * t38175;
    let t38177 = t10841 * t1607;
    let t38182 = t2228 * t505;
    let t38183 = t38182 * t539;
    let t38185 = t10856 * t6245;
    let t38189 = t783 * t252 * t6262 * t3320;
    (t38176, t38177, t38182, t38183, t38185, t38189)
}
