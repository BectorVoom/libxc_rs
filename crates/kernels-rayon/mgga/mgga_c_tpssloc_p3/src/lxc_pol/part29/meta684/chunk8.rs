//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2334/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2334(t2154: f64, t45349: f64, t27776: f64, t95772: f64, t11147: f64, t497: f64, t225: f64, t27424: f64, t27422: f64, t24574: f64, t27752: f64, t1252: f64, t14165: f64, t15787: f64, t15793: f64, t24601: f64, t24888: f64, t27406: f64, t27784: f64, t27830: f64, t3471: f64, t3631: f64, t466: f64, t498: f64, t7283: f64, t7300: f64, t7351: f64, t8002: f64, t8010: f64, t85674: f64, t85750: f64, t86501: f64, t94796: f64, t95707: f64) -> f64 {
    let t95884 = t45349 * t2154;
    let t95889 = 0.24369393582936687948e-2_f64 * t95772 * t27776;
    let t95890 = t497 * t11147;
    let t95899 = t27424 * t225;
    let t95902 = t27422 * t225;
    let t95912 = 0.54831135561607547884e-2_f64 * t24574 * t27752;
    let t95913 = -0.49348022005446793095e-1_f64 * t7283 * t7300 * t85674 * t15793 - 0.27415567780803773942e-2_f64 * t7283 * t85750 * t8002 + 24.0_f64 * t27784 * t95884 * t15793 + t95889 - 0.8529287754027840782e-2_f64 * t94796 * t24601 * t95890 * t14165 + 0.14621636149762012769e-1_f64 * t27406 * t24888 + t466 * t95707 * t498 - 2.0_f64 * t95899 * t1252 - 2.0_f64 * t95902 * t1252 - 0.82246703342411321825e-2_f64 * t7283 * t3471 * t8010 - t7351 * t15787 - 0.36554090374405031922e-2_f64 * t86501 - t27830 * t3631 - t95912;
    t95913
}
