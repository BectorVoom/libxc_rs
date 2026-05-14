//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1042/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1042<F: Float>(t1814: F, t2217: F, t2385: F, t939: F, t1410: F, t157: F, t1938: F, t2146: F, t2152: F, t33031: F, t33037: F, t33047: F, t33053: F, t38073: F, t38077: F, t38085: F, t38089: F, t38104: F, t38111: F, t8316: F, t8400: F, t8791: F, t9003: F, t9440: F) -> (F, F) {
    let t41089 = t2217 * t1814;
    let t41106 = t939 * t2385;
    let t41111 = t33031 + 0.17347256376410398924e1 * t38073 - 0.13170898365871023197e1 * t38077 + 0.4336814094102599731e0 * t2146 * t2152 * t41089 * t157 + t33037 - t38085 + t38089 + 0.17347256376410398924e1 * t9003 * t9440 - 0.65854491829355115987e0 * t8316 * t1938 - 0.52041769129231196772e1 * t38104 + 0.13170898365871023197e1 * t33047 + 0.8673628188205199462e0 * t2146 * t2152 * t2385 * t1410 * t157 + 0.17347256376410398924e1 * t33053 - 0.17347256376410398924e1 * t8400 * t41106 * t8791 - 0.17347256376410398924e1 * t38111;
    (t41089, t41111)
}
