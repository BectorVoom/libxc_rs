//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2016/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2016<F: Float>(t99085: F, t99091: F, t93026: F, t93028: F, t93031: F, t93035: F, t93043: F, t93045: F, t93049: F, t93055: F, t93058: F, t99081: F) -> F {
    let t103324 = F::cast_from(0.2032800112371413129e-3_f64) * t99085;
    let t103329 = F::cast_from(0.1219527626469539185e-2_f64) * t99091;
    let t103335 = F::cast_from(0.17149607247227894789e-2_f64) * t99081 + t103324 + F::cast_from(0.10164000561857065645e-3_f64) * t93026 + F::cast_from(0.40015750243531754507e-2_f64) * t93028 - F::cast_from(0.22866142996303859718e-3_f64) * t93031 + F::cast_from(0.10841600599314203355e-2_f64) * t93035 - t103329 - F::cast_from(0.50820002809285328225e-4_f64) * t93043 + F::cast_from(0.40015750243531754507e-2_f64) * t93045 - F::cast_from(0.45351183609335988442e-1_f64) * t93049 - F::cast_from(0.80031500487063509014e-2_f64) * t93055 - F::cast_from(0.50820002809285328225e-4_f64) * t93058;
    t103335
}
