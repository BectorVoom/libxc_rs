//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1363/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1363<F: Float>(t23384: F, t23595: F, t10166: F, t10181: F, t1052: F, t1065: F, t11084: F, t1955: F, t1956: F, t23327: F, t23329: F, t23341: F, t23354: F, t23394: F, t23588: F, t23594: F, t23721: F, t3010: F, t3016: F, t3026: F, t3174: F, t43440: F, t43604: F, t6680: F, t6687: F, t6704: F, t6705: F, t82442: F, t82457: F, t82463: F, t82469: F, t82481: F, t884: F, t986: F) -> F {
    let t82490 = t23384 * t23595;
    let t82492 = -F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t986 * t82442 + F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t3016 * t23588 + F::cast_from(6.0_f64) * t1052 * t3174 * t23721 * t1065 + F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t6704 * t23394 * t10181 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t23329 * t82457 * t884 + F::cast_from(0.82246703342411321826e-2_f64) * t82463 + F::cast_from(24.0_f64) * t1052 * t43604 * t1955 * t10166 + F::cast_from(0.10966227112321509577e-1_f64) * t6687 * t82469 * t23594 + F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t3010 * t23588 - F::cast_from(0.65797362673929057459e-1_f64) * t6680 * t23354 - F::cast_from(18.0_f64) * t3026 * t23341 - t43440 * t1956 - F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t6704 * t82481 * t10166 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t6704 * t6705 * t11084 + F::cast_from(0.36554090374405031922e-2_f64) * t82490;
    t82492
}
