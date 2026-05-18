//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1339/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1339<F: Float>(t4166: F, t6712: F, t10789: F, t10792: F, t10829: F, t2251: F, t2252: F, t2267: F, t2273: F, t24816: F, t24923: F, t28730: F, t28733: F, t28736: F, t28739: F, t28744: F, t4167: F, t4170: F, t6636: F, t6683: F, t6710: F, t6716: F, t8760: F, t8770: F, t8788: F, t8791: F, t8798: F, t8802: F, t8821: F, t8824: F, t8862: F, t8916: F) -> F {
    let t29202 = t4166 * t6712;
    let t29214 = -F::new(0.46785788981077169656e1) * t8824 * t8760 + F::new(0.69263436422725855034e2) * t8916 * t8770 + F::new(0.70178683471615754484e1) * t6636 * t10789 - F::new(0.46785788981077169656e1) * t6716 * t10792 + t28730 + t28733 + t28736 - t28739 - t28744 + F::new(6.0) * t2273 * t4167 * t2252 - F::new(0.19298375398431042081e3) * t6683 * t4170 * t2267 - F::new(2.0) * t2251 * t4167 * t2267 - F::new(0.19298375398431042081e3) * t6683 * t10829 * t2252 + F::new(0.32163958997385070134e2) * t2273 * t10829 * t2267 + F::new(0.2069040516770936012e4) * t6710 * t29202 * t2252 - F::new(4.0) * t8821 * t8788 - F::new(0.38596750796862084161e3) * t24816 * t8791 + F::new(0.64327917994770140268e2) * t8862 * t8798 + F::new(0.4138081033541872024e4) * t24923 * t8802;
    t29214
}
