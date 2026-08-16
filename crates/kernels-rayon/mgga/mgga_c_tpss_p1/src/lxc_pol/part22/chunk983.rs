//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 983/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk983(t1342: f64, t8079: f64, t2206: f64, t3557: f64, t2215: f64, t10514: f64, t10518: f64, t10520: f64, t10521: f64, t10522: f64, t10523: f64, t10524: f64, t10526: f64, t10528: f64, t10552: f64, t198: f64, t2439: f64, t3728: f64, t740: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t8112: f64, t8117: f64, t8126: f64) -> (f64, f64, f64, f64) {
    let t10557 = 4.0_f64 * t8079 * t1342;
    let t10558 = t3557 * t2206;
    let t10559 = 0.5848223622634646207e0_f64 * t10558;
    let t10560 = t3557 * t2215;
    let t10561 = 0.17315859105681463759e2_f64 * t10560;
    let t10562 = -6.0_f64 * t10514 * t2439 * t3728 + 3.0_f64 * t10552 * t198 * t740 + t10518 + t10520 + t10521 + t10522 + t10523 - t10524 + t10526 + t10528 + t10557 - t10559 - t10561 - t7954 - t7960 + t7972 + t7975 + t8112 - t8117 - t8126;
    (t10557, t10559, t10561, t10562)
}
