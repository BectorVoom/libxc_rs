//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 670/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk670<F: Float>(t1008: F, t422: F, t379: F, t5570: F, t23701: F, t23705: F, t23711: F, t23715: F, t23732: F, t23789: F, t23817: F, t23832: F, t25710: F, t25715: F, t25719: F, t26635: F, t26671: F, t26692: F, t26696: F, t26701: F, t26706: F, t26716: F, t8859: F) -> (F, F) {
    let t26721 = t422 * t1008;
    let t26722 = t26721 * t379;
    let t26723 = t5570 * t26722;
    let t26728 = -F::cast_from(0.33339000546296296298e-1_f64) * t23789 - F::cast_from(0.55565000910493827163e-2_f64) * t23817 - F::cast_from(0.40279602951224778275e-1_f64) * t23701 * t25715 - F::cast_from(0.22226000364197530865e-1_f64) * t26692 * t25719 + F::cast_from(0.33339000546296296297e-1_f64) * t23705 * t5570 * t26696 - F::cast_from(0.33339000546296296298e-1_f64) * t23715 * t5570 * t26701 + F::cast_from(0.33339000546296296298e-1_f64) * t23705 * t5570 * t26706 + F::cast_from(0.33339000546296296298e-1_f64) * t26692 * t25710 + F::cast_from(0.40279602951224778275e-1_f64) * t23711 * t25715 + F::cast_from(0.20003400327777777778e0_f64) * t23732 * t26716 + F::cast_from(0.24167761770734866964e0_f64) * t23832 * t26635 - F::cast_from(0.33339000546296296297e-1_f64) * t23715 * t26723 - F::cast_from(0.10947790369858991997e1_f64) * t8859 * t26671;
    (t26722, t26728)
}
