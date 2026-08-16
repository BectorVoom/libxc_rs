//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1248/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1248(t18039: f64, t18063: f64, t18067: f64, t18071: f64, t18073: f64, t18079: f64, t18084: f64, t18089: f64, t21807: f64, t21822: f64, t2899: f64, t2919: f64, t2922: f64, t301: f64, t5704: f64, t5945: f64, t5961: f64, t757: f64, t758: f64, t761: f64, t7707: f64, t7770: f64) -> f64 {
    let t21837 = 0.14291339372689912324e-3_f64 * t18039 + 0.27439371595564631662e-1_f64 * t7707 * t7770 + 0.12862205435420921092e-2_f64 * t2922 * t21807 * t5961 - 0.25724410870841842183e-2_f64 * t2899 * t21807 * t5704 + 0.21437009059034868486e-3_f64 * t757 * t758 * t301 * t21822 * t761 + 0.21722835846488666732e-1_f64 * t5945 * t2919 - 5.0_f64 / 432.0_f64 * t18063 + t18067 / 144.0_f64 - t18071 / 288.0_f64 - 11.0_f64 / 108.0_f64 * t18073 - t18079 / 18.0_f64 - t18084 / 27.0_f64 - t18089 / 72.0_f64;
    t21837
}
