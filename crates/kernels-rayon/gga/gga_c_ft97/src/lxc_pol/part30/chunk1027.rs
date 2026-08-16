//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1027/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1027(t123125: f64, t141053: f64, t141060: f64, t141071: f64, t141171: f64, t141172: f64, t141176: f64, t150580: f64, t150591: f64, t150594: f64, t150602: f64, t150603: f64, t17864: f64, t17987: f64, t2035: f64, t33359: f64, t33394: f64, t3817: f64, t66076: f64, t709: f64, t7205: f64, t7590: f64, t98545: f64) -> f64 {
    let t150611 = 0.10338048737805743097e-3_f64 * t123125 * t33359 - 0.13200366700519885118e-5_f64 * t141171 * t141172 * t150580 + 0.29693535778629056444e-3_f64 * t141176 * t98545 * t150580 + 0.22979081259345929704e-6_f64 * t66076 * t33394 * t17864 + 0.26043295784446077722e-6_f64 * t150591 + 0.26350381008313446725e-3_f64 * t17987 * t150594 - 0.19762785756235085044e-4_f64 * t17987 * t2035 * t7590 * t3817 - 0.12690037786211307469e-3_f64 * t150602 * t7205 * t150603 * t709 + 0.37842536879785276493e-4_f64 * t141053 - 0.45497819271775541929e-4_f64 * t141060 - 0.85124811172839506173e-2_f64 * t141071;
    t150611
}
