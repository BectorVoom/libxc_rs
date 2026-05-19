//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1157/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1157<F: Float>(t32275: F, t32707: F, t94801: F, t122295: F, t94390: F, t28911: F, t8584: F, t25875: F, t25901: F, t32268: F, t2470: F, t32706: F) -> (F, F, F, F, F, F, F) {
    let t122299 = t94801 * t32275 * t32707;
    let t122309 = F::cast_from(0.50779446784275991476e-2_f64) * t94390 * t32275 * t122295;
    let t122310 = t8584 * t28911;
    let t122311 = t25875 * t122310;
    let t122312 = t122311 * t25901;
    let t122314 = t32268 * t122310;
    let t122315 = t122314 * t25901;
    let t122317 = t32706 * t2470;
    (t122299, t122309, t122311, t122312, t122314, t122315, t122317)
}
