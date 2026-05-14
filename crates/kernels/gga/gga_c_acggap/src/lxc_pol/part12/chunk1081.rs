//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1081/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1081<F: Float>(t38092: F, t7963: F, t7965: F, t4210: F, t7942: F, t2385: F, t323: F, t851: F, t7990: F, t9154: F, t862: F, t865: F, t1410: F, t157: F, t2146: F, t2152: F, t2217: F, t33175: F, t33201: F, t33208: F, t33706: F, t33778: F, t36511: F, t406: F, t7912: F, t7931: F, t8306: F, t8307: F, t8351: F, t8400: F, t8402: F, t9003: F, t9367: F, t9440: F) -> (F,) {
    let t38280 = 0.17347256376410398924e1 * t7963 * t38092 * t7965;
    let t38283 = 0.17347256376410398924e1 * t7942 * t38092 * t4210;
    let t38285 = t851 * t2385 * t323;
    let t38293 = 0.34694512752820797848e1 * t7990 * t9154;
    let t38309 = t862 * t2385 * t865;
    let t38311 = 0.17347256376410398924e1 * t7912 * t9440 + t33201 - 0.17347256376410398924e1 * t33778 * t8307 + 0.8673628188205199462e0 * t8400 * t33175 * t8402 + t38280 - t38283 - 0.13170898365871023197e1 * t38285 + 0.4336814094102599731e0 * t9003 * t8351 + 0.4336814094102599731e0 * t8400 * t8306 * t33706 - t38293 - 0.13170898365871023197e1 * t33208 - 0.17347256376410398924e1 * t7931 * t8306 * t36511 + 0.8673628188205199462e0 * t2146 * t2152 * t2217 * t1410 * t157 + 0.8673628188205199462e0 * t2146 * t2152 * t9367 * t406 * t157 + 0.13170898365871023197e1 * t38309;
    (t38311,)
}
