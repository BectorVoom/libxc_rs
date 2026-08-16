//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1984/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1984(t3999: f64, t8085: f64, t102397: f64, t102634: f64, t102636: f64, t102661: f64, t14224: f64, t14230: f64, t1903: f64, t22387: f64, t25930: f64, t26282: f64, t26304: f64, t27868: f64, t28899: f64, t28911: f64, t30252: f64, t5728: f64, t6896: f64, t75047: f64, t75051: f64, t7511: f64, t75305: f64, t94705: f64, t96546: f64, t96549: f64) -> f64 {
    let t109731 = t3999 * t8085;
    let t109756 = t102634 - 0.17347256376410398924e1_f64 * t25930 * t102661 * t14224 - 0.65854491829355115987e0_f64 * t7511 * t22387 - 0.17347256376410398924e1_f64 * t27868 * t109731 * t14230 + 0.4336814094102599731e0_f64 * t27868 * t26304 * t75305 - 0.17347256376410398924e1_f64 * t94705 * t30252 + 0.13170898365871023197e1_f64 * t26282 * t6896 - 0.4818682326780666368e-3_f64 * t102636 + 0.26020884564615598386e1_f64 * t27868 * t102397 * t75047 - 0.26020884564615598386e1_f64 * t27868 * t28911 * t75051 + 0.48186823267806663678e-3_f64 * t96546 + 0.34694512752820797848e1_f64 * t25930 * t28911 * t1903 * t14230 + t96549 + 0.26341796731742046394e1_f64 * t28899 * t5728;
    t109756
}
