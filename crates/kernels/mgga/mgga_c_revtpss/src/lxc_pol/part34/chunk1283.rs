//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1283/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1283<F: Float>(t106388: F, t10871: F, t113141: F, t113242: F, t1949: F, t213: F, t225: F, t23383: F, t25391: F, t257: F, t27189: F, t27199: F, t29611: F, t29659: F, t29682: F, t29683: F, t29695: F, t6049: F, t7070: F, t7071: F, t7766: F, t93278: F, t93355: F, t99191: F, t99334: F, t99366: F, t99381: F, t99412: F, t99423: F) -> F {
    let t113351 = F::new(0.8673628188205199462e0) * t7070 * t7071 * t1949 * t23383 + t93278 - F::new(0.26020884564615598386e1) * t27199 * t29695 - F::new(0.52041769129231196772e1) * t25391 * t99334 * t29682 - F::new(0.10281140612419229762e0) * t99366 - F::new(0.52041769129231196772e1) * t99191 * t29683 + F::new(0.26020884564615598386e1) * t7070 * t93355 * t113141 * t10871 + F::new(0.65854491829355115987e0) * t213 * t113242 * t225 * t257 + F::new(0.39512695097613069591e1) * t27189 * t6049 + F::new(0.51405703062096148814e-2) * t99381 + F::new(0.52041769129231196772e1) * t27199 * t29611 + F::new(0.57824187921367996415e-1) * t99412 - F::new(0.13010442282307799193e1) * t7766 * t29659 - F::new(0.38554277296572111609e-1) * t106388 + F::new(0.14456046980341999104e-2) * t99423;
    t113351
}
