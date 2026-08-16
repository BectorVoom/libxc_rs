//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2432/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432<F: Float>(t1041: F, t13969: F, t14188: F, t1046: F, t10898: F, t10949: F, t13977: F, t13982: F, t13987: F, t1618: F, t3043: F, t42595: F, t43120: F, t43322: F, t43343: F, t4596: F, t4652: F, t49721: F, t49732: F, t49734: F, t49740: F, t49743: F) -> F {
    let t49748 = t1041 * t13969 * t14188;
    let t49750 = t49721 / F::cast_from(1536.0_f64) + t43343 * t4596 / F::cast_from(512.0_f64) + t10949 * t13977 / F::cast_from(256.0_f64) + t10949 * t13982 / F::cast_from(512.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t43322 * t13987 + t49732 / F::cast_from(48.0_f64) + t49734 / F::cast_from(1536.0_f64) - t10898 * t4652 / F::cast_from(96.0_f64) - t43120 * t1618 / F::cast_from(192.0_f64) - t49740 * t1046 / F::cast_from(144.0_f64) + t49743 * t3043 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(7776.0_f64) * t42595 + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t49748;
    t49750
}
