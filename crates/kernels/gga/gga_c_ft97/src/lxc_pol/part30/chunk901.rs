//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 901/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk901<F: Float>(t108531: F, t6757: F, t35385: F, t6050: F, t30671: F, t2035: F, t35924: F, t709: F, t224: F, t6793: F, t9682: F, t213: F, t665: F, t123125: F, t141053: F, t141060: F, t141071: F, t141171: F, t141172: F, t141176: F, t17864: F, t17987: F, t33359: F, t33394: F, t3817: F, t66076: F, t7205: F, t7590: F, t98545: F) -> (F, F, F, F) {
    let t150580 = t6757 * t108531;
    let t150590 = t35385 * t6050;
    let t150591 = t30671 * t150590;
    let t150594 = t2035 * t35924 * t709;
    let t150602 = t224 * t9682 * t6793;
    let t150603 = t665 * t213;
    let t150611 = 0.10338048737805743097e-3 * t123125 * t33359 - 0.13200366700519885118e-5 * t141171 * t141172 * t150580 + 0.29693535778629056444e-3 * t141176 * t98545 * t150580 + 0.22979081259345929704e-6 * t66076 * t33394 * t17864 + 0.26043295784446077722e-6 * t150591 + 0.26350381008313446725e-3 * t17987 * t150594 - 0.19762785756235085044e-4 * t17987 * t2035 * t7590 * t3817 - 0.12690037786211307469e-3 * t150602 * t7205 * t150603 * t709 + 0.37842536879785276493e-4 * t141053 - 0.45497819271775541929e-4 * t141060 - 0.85124811172839506173e-2 * t141071;
    (t150590, t150594, t150603, t150611)
}
