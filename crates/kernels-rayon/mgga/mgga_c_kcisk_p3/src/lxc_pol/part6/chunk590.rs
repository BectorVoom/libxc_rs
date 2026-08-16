//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 590/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk590(t4203: f64, t8244: f64, t470: f64, t7906: f64, t487: f64, t1487: f64, t1284: f64, t8010: f64, t486: f64, t382: f64, t7831: f64, t467: f64, t8161: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8245 = t4203 * t8244;
    let t8247 = t470 * t7906;
    let t8248 = t487 * t8247;
    let t8249 = t1487 * t8248;
    let t8251 = t1284 * t8010;
    let t8252 = t487 * t8251;
    let t8253 = t486 * t8252;
    let t8255 = t382 * t7831;
    let t8256 = t487 * t8255;
    let t8257 = t486 * t8256;
    let t8259 = t8161 * t467;
    (t8245, t8247, t8248, t8249, t8251, t8252, t8253, t8255, t8256, t8257, t8259)
}
