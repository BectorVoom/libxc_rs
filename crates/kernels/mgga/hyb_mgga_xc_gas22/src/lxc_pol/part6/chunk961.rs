//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 961/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk961<F: Float>(t2251: F, t2273: F, t271: F, t3371: F, t3390: F, t6678: F, t6683: F, t6710: F, t6722: F, t8623: F, t8627: F, t8725: F, t8733: F, t8785: F, t8788: F, t8791: F, t8795: F, t8798: F, t8802: F, t8810: F) -> F {
    let t8813 = -t8623 - t8627 - F::new(4.0) * t6722 * t3371 + F::new(0.64327917994770140268e2) * t6678 * t3390 - F::new(4.0) * t2251 * t8785 - F::new(2.0) * t2251 * t8788 - F::new(0.19298375398431042081e3) * t6683 * t8791 + F::new(0.64327917994770140268e2) * t2273 * t8795 + F::new(0.32163958997385070134e2) * t2273 * t8798 + F::new(0.2069040516770936012e4) * t6710 * t8802 - F::new(0.310907e-1) * t8810 * t271 + t8725 - t8733;
    t8813
}
