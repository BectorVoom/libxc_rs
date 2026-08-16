//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2301/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2301(t210: f64, t24848: f64, t27505: f64, t24574: f64, t27466: f64, t3507: f64, t8054: f64, t27455: f64, t1409: f64, t24849: f64, t24851: f64, t24853: f64, t24860: f64, t27406: f64, t27460: f64, t27725: f64, t3248: f64, t3252: f64, t3493: f64, t3604: f64, t3610: f64, t3612: f64, t7283: f64, t7362: f64, t7376: f64, t85984: f64, t85986: f64) -> (f64, f64) {
    let t95092 = t27505 * t210 * t24848;
    let t95098 = 0.18277045187202515961e-2_f64 * t24574 * t27466;
    let t95109 = t8054 * t3507;
    let t95114 = 0.54831135561607547884e-2_f64 * t24574 * t27455;
    let t95122 = 0.14621636149762012769e-1_f64 * t95092 * t24853 + 0.14621636149762012769e-1_f64 * t27406 * t24860 - t95098 - 0.27415567780803773942e-2_f64 * t7283 * t7362 * t27460 * t3252 - 0.54831135561607547884e-2_f64 * t7283 * t7362 * t27460 * t3248 + 2.0_f64 * t3604 * t27725 + 2.0_f64 * t3610 * t95109 * t3612 - t95114 + 0.54831135561607547884e-2_f64 * t85984 - 0.27415567780803773942e-2_f64 * t24849 * t24851 * t1409 * t3493 * t7376 + 0.36554090374405031922e-2_f64 * t85986;
    (t95109, t95122)
}
