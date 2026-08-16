//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1034/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1034(t150751: f64, t3762: f64, t6789: f64, t695: f64, t224: f64, t150688: f64, t70: f64, t705: f64, t1113: f64, t123768: f64, t140919: f64, t140937: f64, t150517: f64, t150727: f64, t150731: f64, t150736: f64, t150740: f64, t17807: f64, t27527: f64, t27625: f64, t27717: f64, t27725: f64, t33362: f64, t33372: f64, t33383: f64, t38176: f64) -> (f64, f64, f64) {
    let t150752 = t150751 * t3762;
    let t150755 = t695 * t6789;
    let t150756 = t224 * t150755;
    let t150757 = t150688 * t70;
    let t150758 = t150757 * t705;
    let t150761 = 0.51690243689028715487e-4_f64 * t27527 * t140919 * t1113 * t3762 + 0.25845121844514357744e-4_f64 * t140937 * t150727 + 0.51690243689028715488e-5_f64 * t27527 * t150731 + 0.1721820212247325051e-5_f64 * t27527 * t150736 + 0.7825932155388508152e-2_f64 * t150740 - 0.17782141943527538963e-1_f64 * t33372 * t27725 + 0.30638775012461239606e-5_f64 * t17807 * t150517 + 0.88910709717637694816e-2_f64 * t123768 * t33362 + 0.21080304806650757379e-3_f64 * t27717 * t38176 * t27625 + 0.13784064983740990796e-3_f64 * t33383 * t150752 + 0.26043295784446077722e-6_f64 * t150756 * t150758;
    (t150752, t150758, t150761)
}
