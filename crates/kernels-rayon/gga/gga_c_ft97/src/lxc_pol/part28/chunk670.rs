//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 670/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk670(t1008: f64, t422: f64, t379: f64, t5570: f64, t23701: f64, t23705: f64, t23711: f64, t23715: f64, t23732: f64, t23789: f64, t23817: f64, t23832: f64, t25710: f64, t25715: f64, t25719: f64, t26635: f64, t26671: f64, t26692: f64, t26696: f64, t26701: f64, t26706: f64, t26716: f64, t8859: f64) -> (f64, f64) {
    let t26721 = t422 * t1008;
    let t26722 = t26721 * t379;
    let t26723 = t5570 * t26722;
    let t26728 = -0.33339000546296296298e-1_f64 * t23789 - 0.55565000910493827163e-2_f64 * t23817 - 0.40279602951224778275e-1_f64 * t23701 * t25715 - 0.22226000364197530865e-1_f64 * t26692 * t25719 + 0.33339000546296296297e-1_f64 * t23705 * t5570 * t26696 - 0.33339000546296296298e-1_f64 * t23715 * t5570 * t26701 + 0.33339000546296296298e-1_f64 * t23705 * t5570 * t26706 + 0.33339000546296296298e-1_f64 * t26692 * t25710 + 0.40279602951224778275e-1_f64 * t23711 * t25715 + 0.20003400327777777778e0_f64 * t23732 * t26716 + 0.24167761770734866964e0_f64 * t23832 * t26635 - 0.33339000546296296297e-1_f64 * t23715 * t26723 - 0.10947790369858991997e1_f64 * t8859 * t26671;
    (t26722, t26728)
}
