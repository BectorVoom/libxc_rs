//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 811/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk811(t1264: f64, t5056: f64, t247: f64, t3629: f64, t5351: f64, t3626: f64, t3627: f64, t471: f64, t1715: f64, t1227: f64, t1261: f64, t1266: f64, t1808: f64, t3625: f64, t3647: f64, t3686: f64, t3705: f64, t5373: f64, t5379: f64, t5381: f64, t5384: f64, t5386: f64, t5391: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5396 = t1264 * t5056;
    let t5397 = t247 * t5396;
    let t5401 = t5351 * t3629;
    let t5402 = t3626 * t5401;
    let t5405 = t3627 * t471;
    let t5406 = t1715 * t5405;
    let t5407 = t3626 * t5406;
    let t5410 = t5373 * t1227 / 108.0_f64 - t3686 / 864.0_f64 - 0.95275595817932748827e-4_f64 * t5379 - 0.14291339372689912324e-3_f64 * t5381 * t1266 + 0.42874018118069736972e-3_f64 * t5384 * t5386 + 0.7622047665434619906e-3_f64 * t5391 * t1266 - 0.14291339372689912324e-3_f64 * t3647 * t1808 - 0.14291339372689912324e-3_f64 * t1261 * t5397 + 0.14291339372689912324e-3_f64 * t3705 - 0.14291339372689912324e-3_f64 * t3625 * t5402 - 0.14291339372689912324e-3_f64 * t3625 * t5407;
    (t5397, t5401, t5402, t5405, t5406, t5407, t5410)
}
