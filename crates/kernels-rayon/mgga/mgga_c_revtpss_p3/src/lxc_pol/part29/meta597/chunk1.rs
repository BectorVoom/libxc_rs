//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2016/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2016(t99085: f64, t99091: f64, t93026: f64, t93028: f64, t93031: f64, t93035: f64, t93043: f64, t93045: f64, t93049: f64, t93055: f64, t93058: f64, t99081: f64) -> f64 {
    let t103324 = 0.2032800112371413129e-3_f64 * t99085;
    let t103329 = 0.1219527626469539185e-2_f64 * t99091;
    let t103335 = 0.17149607247227894789e-2_f64 * t99081 + t103324 + 0.10164000561857065645e-3_f64 * t93026 + 0.40015750243531754507e-2_f64 * t93028 - 0.22866142996303859718e-3_f64 * t93031 + 0.10841600599314203355e-2_f64 * t93035 - t103329 - 0.50820002809285328225e-4_f64 * t93043 + 0.40015750243531754507e-2_f64 * t93045 - 0.45351183609335988442e-1_f64 * t93049 - 0.80031500487063509014e-2_f64 * t93055 - 0.50820002809285328225e-4_f64 * t93058;
    t103335
}
