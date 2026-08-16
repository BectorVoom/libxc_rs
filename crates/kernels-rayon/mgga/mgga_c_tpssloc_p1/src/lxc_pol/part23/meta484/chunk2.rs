//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1474/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1474(t11678: f64, t1214: f64, t1735: f64, t19083: f64, t21776: f64, t22012: f64, t22185: f64, t22309: f64, t248: f64, t3577: f64, t3578: f64, t44725: f64, t44863: f64, t45250: f64, t4889: f64, t5024: f64, t53238: f64, t53440: f64, t5979: f64, t6203: f64, t6225: f64, t66545: f64, t73084: f64, t73096: f64, t73099: f64, t73102: f64, t79018: f64) -> f64 {
    let t79349 = -t73084 / 576.0_f64 - 2.0_f64 / 81.0_f64 * t66545 - t73096 / 384.0_f64 + 5.0_f64 / 1728.0_f64 * t73099 - 5.0_f64 / 216.0_f64 * t19083 * t6203 - 5.0_f64 / 108.0_f64 * t5024 * t22185 + t53238 * t22309 / 128.0_f64 + t44863 * t248 * t1214 * t79018 * t44725 / 128.0_f64 - t73102 / 72.0_f64 - t3577 * t3578 * t1735 * t21776 / 1152.0_f64 - t11678 * t3578 * t6225 * t5979 / 384.0_f64 - t45250 - 5.0_f64 / 972.0_f64 * t53440 + 28.0_f64 / 243.0_f64 * t4889 * t22012;
    t79349
}
