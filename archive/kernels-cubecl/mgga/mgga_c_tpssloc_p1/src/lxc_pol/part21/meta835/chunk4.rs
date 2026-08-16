//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2966/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966<F: Float>(t10408: F, t10413: F, t13977: F, t13982: F, t13987: F, t13991: F, t14099: F, t14103: F, t14508: F, t14511: F, t17673: F, t17693: F, t3041: F, t3048: F, t3070: F, t3071: F, t42432: F, t42561: F, t4347: F, t4650: F, t48548: F, t48564: F, t48567: F, t48570: F, t48574: F, t50265: F, t5677: F) -> F {
    let t61835 = t3070 * t3071 * t4650 * t4347 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t48548 + t14508 * t13982 / F::cast_from(768.0_f64) + t48570 * t13987 / F::cast_from(256.0_f64) - t50265 * t13991 / F::cast_from(256.0_f64) - t14511 * t14103 / F::cast_from(1536.0_f64) - t42561 * t17673 / F::cast_from(48.0_f64) + t14508 * t13977 / F::cast_from(384.0_f64) - t14511 * t14099 / F::cast_from(768.0_f64) + t48564 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(648.0_f64) * t3048 * t17693 + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t48567 + t48574 / F::cast_from(2304.0_f64) - t42432 / F::cast_from(20736.0_f64) - F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t10413 * t10408 * t5677 * t3041;
    t61835
}
