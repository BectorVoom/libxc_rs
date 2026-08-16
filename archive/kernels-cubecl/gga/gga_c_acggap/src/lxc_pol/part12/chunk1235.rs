//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1235/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1235<F: Float>(t119: F, t1264: F, t150: F, t1620: F, t187: F, t2146: F, t2222: F, t2394: F, t31965: F, t32124: F, t33180: F, t33185: F, t33198: F, t36547: F, t38001: F, t38241: F, t38251: F, t38256: F, t38259: F, t5332: F, t7912: F, t8004: F, t8306: F, t8316: F, t9145: F, t9165: F) -> F {
    let t38270 = F::cast_from(0.17347256376410398924e1_f64) * t33180 + t33185 + t38241 - F::cast_from(0.26020884564615598386e1_f64) * t2146 * t8004 * t2394 * t1264 - F::cast_from(0.65854491829355115987e0_f64) * t2222 * t5332 + F::cast_from(0.8673628188205199462e0_f64) * t7912 * t9145 - t38251 - F::cast_from(0.17347256376410398924e1_f64) * t31965 * t9165 + t38256 - t38259 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t38001 * t150 * t187 + F::cast_from(0.26020884564615598386e1_f64) * t32124 * t8306 * t36547 + F::cast_from(0.26341796731742046394e1_f64) * t8316 * t1620 - F::cast_from(0.17347256376410398924e1_f64) * t33198;
    t38270
}
