//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1019/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1019(t24260: f64, t3766: f64, t92354: f64, t108795: f64, t420: f64, t6018: f64, t6818: f64, t108826: f64, t123408: f64, t140871: f64, t140894: f64, t141096: f64, t141097: f64, t27507: f64, t27516: f64, t27524: f64, t27553: f64, t27633: f64, t27638: f64, t27653: f64, t27661: f64, t33366: f64, t33380: f64, t33404: f64, t6037: f64, t9: f64, t96510: f64) -> f64 {
    let t150398 = t3766 * t24260 * t92354;
    let t150413 = t108795 * t420;
    let t150419 = t3766 * t6018 * t92354;
    let t150420 = t6818 * t420;
    let t150428 = 0.6809984893827160494e-1_f64 * t33380 * t27507 + 0.36328835404593865432e-2_f64 * t150398 * t27524 - 0.10338048737805743097e-3_f64 * t108826 * t27553 - 0.45967398033333333333e0_f64 * t3766 * t96510 * t9 * t27516 + 0.22705522127871165896e-3_f64 * t123408 * t33404 * t27653 + 0.51074886703703703703e-1_f64 * t141096 * t141097 * t27638 - 0.23754828622903245156e-2_f64 * t33366 * t150413 * t6037 + 0.29693535778629056444e-3_f64 * t140871 - 0.60548059007656442387e-3_f64 * t150419 * t150420 * t27661 - 0.13200366700519885118e-5_f64 * t140894 - 0.34049924469135802469e-1_f64 * t33380 * t141097 * t27633;
    t150428
}
