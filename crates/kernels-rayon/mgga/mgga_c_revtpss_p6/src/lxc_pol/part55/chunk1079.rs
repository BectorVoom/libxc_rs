//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1079/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1079(t33245: f64, t33257: f64, t33261: f64, t33314: f64, t3: f64, t1461: f64, t2115: f64, t2170: f64, t32373: f64, t32377: f64, t32760: f64, t32762: f64, t32764: f64, t32772: f64, t32775: f64, t32778: f64, t32781: f64, t573: f64, t7554: f64, t7557: f64, t7696: f64, t8616: f64, t8905: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t33316 = 2.0_f64 * t33245 + t33257 + t33261 + t33314;
    let t33317 = t3 * t33316;
    let t33328 = param_d * t33316;
    let t33338 = 3.0_f64 * t1461 * t8905 + 3.0_f64 * t2115 * t7696 + 6.0_f64 * t2170 * t7554 + 3.0_f64 * t2170 * t7557 + t33328 * t573 + t32373 + t32377 + t32760 + t32762 + t32764 + t32772 + t32775 + t32778 + t32781 + t8616;
    (t33316, t33317, t33328, t33338)
}
