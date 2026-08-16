//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1339/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1339(t4166: f64, t6712: f64, t10789: f64, t10792: f64, t10829: f64, t2251: f64, t2252: f64, t2267: f64, t2273: f64, t24816: f64, t24923: f64, t28730: f64, t28733: f64, t28736: f64, t28739: f64, t28744: f64, t4167: f64, t4170: f64, t6636: f64, t6683: f64, t6710: f64, t6716: f64, t8760: f64, t8770: f64, t8788: f64, t8791: f64, t8798: f64, t8802: f64, t8821: f64, t8824: f64, t8862: f64, t8916: f64) -> f64 {
    let t29202 = t4166 * t6712;
    let t29214 = -0.46785788981077169656e1_f64 * t8824 * t8760 + 0.69263436422725855034e2_f64 * t8916 * t8770 + 0.70178683471615754484e1_f64 * t6636 * t10789 - 0.46785788981077169656e1_f64 * t6716 * t10792 + t28730 + t28733 + t28736 - t28739 - t28744 + 6.0_f64 * t2273 * t4167 * t2252 - 0.19298375398431042081e3_f64 * t6683 * t4170 * t2267 - 2.0_f64 * t2251 * t4167 * t2267 - 0.19298375398431042081e3_f64 * t6683 * t10829 * t2252 + 0.32163958997385070134e2_f64 * t2273 * t10829 * t2267 + 0.2069040516770936012e4_f64 * t6710 * t29202 * t2252 - 4.0_f64 * t8821 * t8788 - 0.38596750796862084161e3_f64 * t24816 * t8791 + 0.64327917994770140268e2_f64 * t8862 * t8798 + 0.4138081033541872024e4_f64 * t24923 * t8802;
    t29214
}
