//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1068/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1068(t14680: f64, t318: f64, t294: f64, t10961: f64, t1425: f64, t4960: f64, t905: f64, t912: f64, t14447: f64, t14449: f64, t14451: f64, t14573: f64, t14575: f64, t14578: f64, t14579: f64, t14583: f64, t14585: f64, t14586: f64, t14636: f64, t14638: f64, t14641: f64, t14658: f64, t14662: f64, t14666: f64, t4023: f64, t993: f64) -> (f64, f64, f64, f64, f64) {
    let t14681 = t14680 * t318;
    let t14683 = 0.19751673498613801407e-1_f64 * t294 * t14681;
    let t14685 = 2.0_f64 * t10961 * t1425;
    let t14686 = t4960 * t905;
    let t14688 = 0.35089341735807877242e1_f64 * t912 * t14686;
    let t14689 = -t14579 * t4023 * t993 + 2.0_f64 * t14586 * t4023 * t993 + t14447 - t14449 + t14451 + t14573 + t14575 + t14578 - t14583 + t14585 - t14636 - t14638 - t14641 - t14658 + t14662 + t14666 + t14683 + t14685 - t14688;
    (t14681, t14683, t14685, t14688, t14689)
}
