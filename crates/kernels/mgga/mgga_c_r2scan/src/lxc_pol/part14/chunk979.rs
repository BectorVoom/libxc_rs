//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 979/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk979<F: Float>(t10833: F, t776: F, t1615: F, t269: F, t2147: F, t2150: F, t507: F, t512: F, t6100: F, t10841: F, t1607: F, t2228: F, t505: F, t539: F, t10856: F, t6245: F) -> (F, F, F, F, F, F, F, F) {
    let t38166 = t776 * t10833;
    let t38168 = t1615 * t269;
    let t38170 = t2147 * t38168 * t2150;
    let t38175 = t512 * t6100 * t507;
    let t38177 = t10841 * t1607;
    let t38182 = t2228 * t505;
    let t38183 = t38182 * t539;
    let t38185 = t10856 * t6245;
    (t38166, t38168, t38170, t38175, t38177, t38182, t38183, t38185)
}
