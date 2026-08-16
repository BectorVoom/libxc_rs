//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1243/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1243(t30525: f64, t30541: f64, t10830: f64, t10834: f64, t17633: f64, t1950: f64, t20908: f64, t30263: f64, t30265: f64, t30268: f64, t30362: f64, t30364: f64, t30366: f64, t30369: f64, t30381: f64, t30385: f64, t30387: f64, t30502: f64, t714: f64, t722: f64, t9423: f64) -> (f64, f64) {
    let t30542 = t30525 + t30541;
    let t30548 = t30263 + t30265 + t30268 - t30362 - t30364 - t30366 - t30369 - t30381 - t30385 - t30387 - t30502 - 0.57895126195293126241e3_f64 * t20908 * t9423 + 0.5848223622634646207e0_f64 * t1950 * t10830 + 0.5848223622634646207e0_f64 * t714 * t30542 * t722 + 0.10254018858216406658e4_f64 * t17633 * t10834;
    (t30542, t30548)
}
