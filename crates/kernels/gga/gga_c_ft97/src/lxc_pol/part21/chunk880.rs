//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 880/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk880<F: Float>(t1013: F, t422: F, t379: F, t538: F, t920: F, t423: F, t554: F, t1008: F, t72: F, t5579: F, t5570: F, t23701: F, t23705: F, t23711: F, t23715: F, t23732: F, t23789: F, t23817: F, t23832: F, t25710: F, t25715: F, t25719: F, t26635: F, t26671: F, t26692: F, t8859: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26695 = t422 * t1013;
    let t26696 = t26695 * t379;
    let t26700 = t920 * t538;
    let t26701 = t423 * t26700;
    let t26705 = t920 * t554;
    let t26706 = t423 * t26705;
    let t26714 = t1008 * t554;
    let t26715 = t72 * t26714;
    let t26716 = t5579 * t26715;
    let t26721 = t422 * t1008;
    let t26722 = t26721 * t379;
    let t26723 = t5570 * t26722;
    let t26728 = -0.33339000546296296298e-1 * t23789 - 0.55565000910493827163e-2 * t23817 - 0.40279602951224778275e-1 * t23701 * t25715 - 0.22226000364197530865e-1 * t26692 * t25719 + 0.33339000546296296297e-1 * t23705 * t5570 * t26696 - 0.33339000546296296298e-1 * t23715 * t5570 * t26701 + 0.33339000546296296298e-1 * t23705 * t5570 * t26706 + 0.33339000546296296298e-1 * t26692 * t25710 + 0.40279602951224778275e-1 * t23711 * t25715 + 0.20003400327777777778e0 * t23732 * t26716 + 0.24167761770734866964e0 * t23832 * t26635 - 0.33339000546296296297e-1 * t23715 * t26723 - 0.10947790369858991997e1 * t8859 * t26671;
    (t26695, t26696, t26700, t26701, t26705, t26706, t26715, t26721, t26722, t26728)
}
