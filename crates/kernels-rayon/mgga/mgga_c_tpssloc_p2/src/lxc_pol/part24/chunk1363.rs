//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1363/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1363(t23384: f64, t23595: f64, t10166: f64, t10181: f64, t1052: f64, t1065: f64, t11084: f64, t1955: f64, t1956: f64, t23327: f64, t23329: f64, t23341: f64, t23354: f64, t23394: f64, t23588: f64, t23594: f64, t23721: f64, t3010: f64, t3016: f64, t3026: f64, t3174: f64, t43440: f64, t43604: f64, t6680: f64, t6687: f64, t6704: f64, t6705: f64, t82442: f64, t82457: f64, t82463: f64, t82469: f64, t82481: f64, t884: f64, t986: f64) -> f64 {
    let t82490 = t23384 * t23595;
    let t82492 = -0.49348022005446793095e-1_f64 * t6687 * t986 * t82442 + 0.24674011002723396548e-1_f64 * t6687 * t3016 * t23588 + 6.0_f64 * t1052 * t3174 * t23721 * t1065 + 0.49348022005446793095e-1_f64 * t6687 * t6704 * t23394 * t10181 - 0.82246703342411321826e-2_f64 * t23327 * t23329 * t82457 * t884 + 0.82246703342411321826e-2_f64 * t82463 + 24.0_f64 * t1052 * t43604 * t1955 * t10166 + 0.10966227112321509577e-1_f64 * t6687 * t82469 * t23594 + 0.24674011002723396548e-1_f64 * t6687 * t3010 * t23588 - 0.65797362673929057459e-1_f64 * t6680 * t23354 - 18.0_f64 * t3026 * t23341 - t43440 * t1956 - 0.49348022005446793095e-1_f64 * t6687 * t6704 * t82481 * t10166 - 0.82246703342411321825e-2_f64 * t6687 * t6704 * t6705 * t11084 + 0.36554090374405031922e-2_f64 * t82490;
    t82492
}
