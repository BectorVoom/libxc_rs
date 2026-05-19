//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1216/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1216<F: Float>(t2138: F, t2147: F, t322: F, t9793: F, t157: F, t2127: F, t2146: F, t2152: F, t29997: F, t32082: F, t32091: F, t32109: F, t32121: F, t33794: F, t33798: F, t33801: F, t33804: F, t36405: F, t36419: F, t39794: F, t6068: F, t609: F, t6569: F, t7931: F, t8400: F, t9033: F, t9508: F) -> F {
    let t40824 = t2138 * t2147 * t9793 * t322;
    let t40837 = F::cast_from(0.13170898365871023197e1_f64) * t2127 * t6569 - t33794 + t33798 - t33801 - t33804 - F::cast_from(0.13170898365871023197e1_f64) * t32082 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t29997 * t9508 - t32091 - F::cast_from(0.17347256376410398924e1_f64) * t40824 - F::cast_from(0.26341796731742046394e1_f64) * t36405 - t32109 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2152 * t609 * t6068 * t157 - F::cast_from(0.26020884564615598386e1_f64) * t8400 * t9033 * t39794 + F::cast_from(0.13170898365871023197e1_f64) * t32121 - F::cast_from(0.13877805101128319139e2_f64) * t36419;
    t40837
}
