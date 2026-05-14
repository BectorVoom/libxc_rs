//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1082/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1082<F: Float>(t32092: F, t9168: F, t33323: F, t557: F, t33092: F, t872: F, t9380: F, t2132: F, t2138: F, t2385: F, t879: F, t2222: F, t2245: F, t33210: F, t33214: F, t33227: F, t33228: F, t33230: F, t33234: F, t33658: F, t4119: F, t639: F, t9058: F) -> (F,) {
    let t38315 = 0.17347256376410398924e1 * t32092 * t9168;
    let t38319 = 0.13170898365871023197e1 * t33323 * t557;
    let t38321 = t33092 * t557;
    let t38324 = 0.13170898365871023197e1 * t9380 * t872;
    let t38329 = t2138 * t2132 * t2385 * t879;
    let t38336 = -0.34694512752820797848e1 * t33210 + t38315 + 0.26341796731742046394e1 * t2222 * t4119 - t38319 + 0.34694512752820797848e1 * t33214 - 0.65854491829355115987e0 * t38321 + t38324 - 0.4336814094102599731e0 * t33658 * t639 - t33227 + 0.8673628188205199462e0 * t38329 + 0.17347256376410398924e1 * t33228 - 0.17347256376410398924e1 * t33230 - 0.8673628188205199462e0 * t9058 * t2245 - 0.34694512752820797848e1 * t33234;
    (t38336,)
}
