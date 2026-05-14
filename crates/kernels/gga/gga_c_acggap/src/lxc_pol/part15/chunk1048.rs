//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1048/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1048<F: Float>(t2176: F, t5517: F, t1614: F, t9380: F, t8397: F, t9159: F, t157: F, t1838: F, t2146: F, t2152: F, t2217: F, t2245: F, t33208: F, t38280: F, t38283: F, t38285: F, t38293: F, t38309: F, t38315: F, t38319: F, t38321: F, t38771: F, t8400: F, t9427: F, t9517: F) -> (F,) {
    let t41250 = t2176 * t5517;
    let t41258 = t9380 * t1614;
    let t41265 = t8397 * t9159;
    let t41267 = t38280 - t38283 - 0.26341796731742046394e1 * t38285 - t38293 - 0.65854491829355115987e0 * t33208 + 0.26341796731742046394e1 * t38309 - 0.13170898365871023197e1 * t41250 - 0.17347256376410398924e1 * t8400 * t9427 * t38771 + t38315 - 0.4336814094102599731e0 * t9517 * t2245 - t38319 - 0.13170898365871023197e1 * t38321 + 0.13170898365871023197e1 * t41258 + 0.4336814094102599731e0 * t2146 * t2152 * t2217 * t1838 * t157 + 0.34694512752820797848e1 * t41265;
    (t41267,)
}
