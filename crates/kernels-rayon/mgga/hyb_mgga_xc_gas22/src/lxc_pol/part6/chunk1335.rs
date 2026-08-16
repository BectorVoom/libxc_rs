//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1335/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1335(t20703: f64, t20706: f64, t20770: f64, t24556: f64, t24559: f64, t24562: f64, t251: f64, t28853: f64, t28856: f64, t28859: f64, t10629: f64, t1371: f64, t20934: f64, t25160: f64, t28949: f64, t28973: f64, t29028: f64, t29031: f64, t29033: f64, t29036: f64, t29038: f64, t29040: f64, t29042: f64, t29044: f64, t29046: f64, t856: f64) -> (f64, f64) {
    let t29057 = 0.621814e-1_f64 * (t20770 - 0.11080740740740740741e0_f64 * t20703 + 0.23744444444444444444e-1_f64 * t20706 - 0.11080740740740740741e0_f64 * t24556 + 0.94977777777777777776e-1_f64 * t24559 - 0.35616666666666666666e-1_f64 * t24562 + 0.23744444444444444444e-1_f64 * t28859 - 0.35616666666666666666e-1_f64 * t28853 + 0.53425e-1_f64 * t28856) * t251;
    let t29061 = -0.14035736694323150897e2_f64 * t25160 * t1371 * t28949 + t28973 + t29028 + t29031 + t29033 + t29036 + t29038 + t29040 + t29042 - t29044 + t29046 - t29057 - 0.10254018858216406658e4_f64 * t856 * t10629 * t20934;
    (t29057, t29061)
}
