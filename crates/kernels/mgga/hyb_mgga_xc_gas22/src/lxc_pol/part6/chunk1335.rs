//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1335/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1335<F: Float>(t20703: F, t20706: F, t20770: F, t24556: F, t24559: F, t24562: F, t251: F, t28853: F, t28856: F, t28859: F, t10629: F, t1371: F, t20934: F, t25160: F, t28949: F, t28973: F, t29028: F, t29031: F, t29033: F, t29036: F, t29038: F, t29040: F, t29042: F, t29044: F, t29046: F, t856: F) -> (F, F) {
    let t29057 = F::new(0.621814e-1) * (t20770 - F::new(0.11080740740740740741e0) * t20703 + F::new(0.23744444444444444444e-1) * t20706 - F::new(0.11080740740740740741e0) * t24556 + F::new(0.94977777777777777776e-1) * t24559 - F::new(0.35616666666666666666e-1) * t24562 + F::new(0.23744444444444444444e-1) * t28859 - F::new(0.35616666666666666666e-1) * t28853 + F::new(0.53425e-1) * t28856) * t251;
    let t29061 = -F::new(0.14035736694323150897e2) * t25160 * t1371 * t28949 + t28973 + t29028 + t29031 + t29033 + t29036 + t29038 + t29040 + t29042 - t29044 + t29046 - t29057 - F::new(0.10254018858216406658e4) * t856 * t10629 * t20934;
    (t29057, t29061)
}
