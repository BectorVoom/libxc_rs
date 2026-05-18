//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1019/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1019<F: Float>(t24260: F, t3766: F, t92354: F, t108795: F, t420: F, t6018: F, t6818: F, t108826: F, t123408: F, t140871: F, t140894: F, t141096: F, t141097: F, t27507: F, t27516: F, t27524: F, t27553: F, t27633: F, t27638: F, t27653: F, t27661: F, t33366: F, t33380: F, t33404: F, t6037: F, t9: F, t96510: F) -> F {
    let t150398 = t3766 * t24260 * t92354;
    let t150413 = t108795 * t420;
    let t150419 = t3766 * t6018 * t92354;
    let t150420 = t6818 * t420;
    let t150428 = F::new(0.6809984893827160494e-1) * t33380 * t27507 + F::new(0.36328835404593865432e-2) * t150398 * t27524 - F::new(0.10338048737805743097e-3) * t108826 * t27553 - F::new(0.45967398033333333333e0) * t3766 * t96510 * t9 * t27516 + F::new(0.22705522127871165896e-3) * t123408 * t33404 * t27653 + F::new(0.51074886703703703703e-1) * t141096 * t141097 * t27638 - F::new(0.23754828622903245156e-2) * t33366 * t150413 * t6037 + F::new(0.29693535778629056444e-3) * t140871 - F::new(0.60548059007656442387e-3) * t150419 * t150420 * t27661 - F::new(0.13200366700519885118e-5) * t140894 - F::new(0.34049924469135802469e-1) * t33380 * t141097 * t27633;
    t150428
}
