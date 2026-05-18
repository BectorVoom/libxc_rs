//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1027/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1027<F: Float>(t123125: F, t141053: F, t141060: F, t141071: F, t141171: F, t141172: F, t141176: F, t150580: F, t150591: F, t150594: F, t150602: F, t150603: F, t17864: F, t17987: F, t2035: F, t33359: F, t33394: F, t3817: F, t66076: F, t709: F, t7205: F, t7590: F, t98545: F) -> F {
    let t150611 = F::new(0.10338048737805743097e-3) * t123125 * t33359 - F::new(0.13200366700519885118e-5) * t141171 * t141172 * t150580 + F::new(0.29693535778629056444e-3) * t141176 * t98545 * t150580 + F::new(0.22979081259345929704e-6) * t66076 * t33394 * t17864 + F::new(0.26043295784446077722e-6) * t150591 + F::new(0.26350381008313446725e-3) * t17987 * t150594 - F::new(0.19762785756235085044e-4) * t17987 * t2035 * t7590 * t3817 - F::new(0.12690037786211307469e-3) * t150602 * t7205 * t150603 * t709 + F::new(0.37842536879785276493e-4) * t141053 - F::new(0.45497819271775541929e-4) * t141060 - F::new(0.85124811172839506173e-2) * t141071;
    t150611
}
