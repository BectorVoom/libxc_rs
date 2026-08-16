//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1238/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1238(t32092: f64, t9168: f64, t33323: f64, t557: f64, t33092: f64, t872: f64, t9380: f64, t2132: f64, t2138: f64, t2385: f64, t879: f64, t2222: f64, t2245: f64, t33210: f64, t33214: f64, t33227: f64, t33228: f64, t33230: f64, t33234: f64, t33658: f64, t4119: f64, t639: f64, t9058: f64) -> f64 {
    let t38315 = 0.17347256376410398924e1_f64 * t32092 * t9168;
    let t38319 = 0.13170898365871023197e1_f64 * t33323 * t557;
    let t38321 = t33092 * t557;
    let t38324 = 0.13170898365871023197e1_f64 * t9380 * t872;
    let t38329 = t2138 * t2132 * t2385 * t879;
    let t38336 = -0.34694512752820797848e1_f64 * t33210 + t38315 + 0.26341796731742046394e1_f64 * t2222 * t4119 - t38319 + 0.34694512752820797848e1_f64 * t33214 - 0.65854491829355115987e0_f64 * t38321 + t38324 - 0.4336814094102599731e0_f64 * t33658 * t639 - t33227 + 0.8673628188205199462e0_f64 * t38329 + 0.17347256376410398924e1_f64 * t33228 - 0.17347256376410398924e1_f64 * t33230 - 0.8673628188205199462e0_f64 * t9058 * t2245 - 0.34694512752820797848e1_f64 * t33234;
    t38336
}
