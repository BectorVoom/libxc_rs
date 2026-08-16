//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1916/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1916(t2770: f64, t387: f64, t3961: f64, t23329: f64, t23581: f64, t7553: f64, t381: f64, t7577: f64, t6691: f64, t1052: f64, t14545: f64, t14552: f64, t1956: f64, t23327: f64, t25400: f64, t25403: f64, t25407: f64, t25410: f64, t25413: f64, t25416: f64, t25420: f64, t25425: f64, t25429: f64, t4660: f64, t4694: f64, t6687: f64, t6771: f64, t6776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25430 = t387 * t2770;
    let t25431 = t25430 * t3961;
    let t25432 = t23329 * t25431;
    let t25436 = t23581 * t7553;
    let t25442 = t7577 * t381;
    let t25443 = t25442 * t6691;
    let t25446 = -t6771 * t4694 - 0.82246703342411321825e-2_f64 * t6687 * t25400 - 0.82246703342411321825e-2_f64 * t6687 * t25403 - 0.82246703342411321825e-2_f64 * t6687 * t25407 - 0.82246703342411321825e-2_f64 * t6687 * t25410 - 0.82246703342411321825e-2_f64 * t6687 * t25413 - 0.27415567780803773942e-2_f64 * t23327 * t25416 + 2.0_f64 * t1052 * t25420 - 0.54831135561607547884e-2_f64 * t23327 * t25425 + 0.36554090374405031923e-2_f64 * t25429 * t25432 - t14552 * t1956 + 0.27415567780803773942e-2_f64 * t6687 * t25436 - t14545 * t1956 + 2.0_f64 * t4660 * t6776 - 0.27415567780803773942e-2_f64 * t23327 * t25443;
    (t25430, t25431, t25432, t25436, t25442, t25443, t25446)
}
