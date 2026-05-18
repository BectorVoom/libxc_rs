//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1079/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1079<F: Float>(t33245: F, t33257: F, t33261: F, t33314: F, t3: F, t1461: F, t2115: F, t2170: F, t32373: F, t32377: F, t32760: F, t32762: F, t32764: F, t32772: F, t32775: F, t32778: F, t32781: F, t573: F, t7554: F, t7557: F, t7696: F, t8616: F, t8905: F) -> (F, F, F, F) {
    let t33316 = F::new(2.0) * t33245 + t33257 + t33261 + t33314;
    let t33317 = t3 * t33316;
    let t33328 = param_d * t33316;
    let t33338 = F::new(3.0) * t1461 * t8905 + F::new(3.0) * t2115 * t7696 + F::new(6.0) * t2170 * t7554 + F::new(3.0) * t2170 * t7557 + t33328 * t573 + t32373 + t32377 + t32760 + t32762 + t32764 + t32772 + t32775 + t32778 + t32781 + t8616;
    (t33316, t33317, t33328, t33338)
}
