//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1257/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1257<F: Float>(t31658: F, t31660: F, t32891: F, t35924: F, t37780: F, t37781: F, t37782: F, t37787: F, t37789: F, t37790: F, t40308: F, t40310: F, t40313: F, t40316: F, t40318: F, t40322: F, t40324: F, t40326: F) -> F {
    let t42046 = t37780 - t37781 + t37782 - F::new(0.83861579438944405515e-2) * t31658 + F::new(0.94344276868812456207e-3) * t31660 + t32891 + F::new(0.17149607247227894789e-2) * t40308 - F::new(0.80031500487063509015e-2) * t40310 + F::new(13.0) / F::new(72.0) * t35924 + t40313 / F::new(12.0) + t40316 / F::new(12.0) + t37787 + F::new(0.34299214494455789578e-2) * t40318 - F::new(0.10718504529517434243e-2) * t40322 + F::new(0.12862205435420921092e-1) * t40324 - t37789 - t37790 + F::new(0.17149607247227894789e-1) * t40326;
    t42046
}
