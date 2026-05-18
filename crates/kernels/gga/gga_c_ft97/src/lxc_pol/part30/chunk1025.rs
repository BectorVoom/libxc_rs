//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1025/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1025<F: Float>(t171: F, t2426: F, t3771: F, t6793: F, t6789: F, t79931: F, t679: F, t123129: F, t123133: F, t141004: F, t141107: F, t150547: F, t150552: F, t150554: F, t150558: F, t17807: F, t27557: F, t27561: F, t27629: F, t27717: F, t33359: F, t33394: F, t33445: F, t35437: F, t6057: F, t689: F, t690: F, t7853: F) -> F {
    let t150565 = t3771 * t2426 * t6793 * t171;
    let t150569 = t79931 * t6789;
    let t150570 = t6793 * t679;
    let t150577 = F::new(0.3827206426927081041e-8) * t17807 * t141107 * t27561 + F::new(0.45958162518691859408e-7) * t17807 * t33394 * t27557 - F::new(0.20869152414369355073e-1) * t33445 * t150547 - F::new(0.60548059007656442387e-3) * t150552 - F::new(0.25537443351851851852e-1) * t150554 * t6057 - F::new(0.45497819271775541929e-4) * t150558 + F::new(0.88910709717637694816e-2) * t27717 * t7853 * t27629 - t141004 - F::new(0.49184261954149446141e-6) * t150565 * t35437 * t690 + F::new(0.24511020009968991683e-5) * t123133 * t150569 * t150570 * t689 + F::new(0.10338048737805743097e-3) * t123129 * t33359;
    t150577
}
