//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 907/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk907<F: Float>(t150755: F, t224: F, t150688: F, t70: F, t705: F, t1113: F, t123768: F, t140919: F, t140937: F, t150517: F, t150727: F, t150731: F, t150736: F, t150740: F, t150752: F, t17807: F, t27527: F, t27625: F, t27717: F, t27725: F, t33362: F, t33372: F, t33383: F, t3762: F, t38176: F) -> (F, F) {
    let t150756 = t224 * t150755;
    let t150757 = t150688 * t70;
    let t150758 = t150757 * t705;
    let t150761 = 0.51690243689028715487e-4 * t27527 * t140919 * t1113 * t3762 + 0.25845121844514357744e-4 * t140937 * t150727 + 0.51690243689028715488e-5 * t27527 * t150731 + 0.1721820212247325051e-5 * t27527 * t150736 + 0.7825932155388508152e-2 * t150740 - 0.17782141943527538963e-1 * t33372 * t27725 + 0.30638775012461239606e-5 * t17807 * t150517 + 0.88910709717637694816e-2 * t123768 * t33362 + 0.21080304806650757379e-3 * t27717 * t38176 * t27625 + 0.13784064983740990796e-3 * t33383 * t150752 + 0.26043295784446077722e-6 * t150756 * t150758;
    (t150758, t150761)
}
