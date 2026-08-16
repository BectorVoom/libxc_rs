//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2191/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2191(t27799: f64, t98779: f64, t1711: f64, t2394: f64, t2430: f64, t27375: f64, t94245: f64, t61155: f64, t2832: f64, t1113: f64, t4537: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t25767: f64, t27364: f64, t27382: f64, t27777: f64, t27802: f64, t27810: f64, t27817: f64, t4541: f64, t51780: f64, t7087: f64, t7091: f64, t7783: f64, t7863: f64, t99542: f64) -> f64 {
    let t101065 = t27799 * t98779;
    let t101070 = t1711 * t2394;
    let t101074 = t1711 * t2430;
    let t101083 = t94245 * t27375;
    let t101086 = t27799 * t61155;
    let t101093 = t1711 * t2832;
    let t101099 = t1113 * t4537;
    let t101105 = t27382 * t101065 + 3.0_f64 * t2403 * t7087 * t27810 - t99542 + 3.0_f64 * t4541 * t1963 * t101070 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t101074 + 3.0_f64 * t2403 * t7087 * t27777 - t1940 * t25440 * t27802 - 3.0_f64 * t25206 * t101083 + 3.0_f64 * t25206 * t101086 - t1940 * t25440 * t27817 + 3.0_f64 * t51780 * t7863 - t1940 * t7091 * t101093 / 2.0_f64 + t1940 * t27364 * t1113 - t1940 * t7091 * t101099 + 3.0_f64 / 2.0_f64 * t2403 * t7783 * t25767;
    t101105
}
