//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1233/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1233(t18704: f64, t19009: f64, t3: f64, t1799: f64, t2061: f64, t116: f64, t5815: f64, t645: f64, t2105: f64, t5953: f64, t117: f64, t18627: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19010 = t18704 + t19009;
    let t19011 = t3 * t19010;
    let t19023 = param_d * t19010;
    let t19037 = t2061 * t1799;
    let t19040 = t116 * t5815;
    let t19041 = t19040 * t645;
    let t19044 = t5953 * t2105;
    let t19047 = t117 * t18627;
    (t19010, t19011, t19023, t19037, t19040, t19041, t19044, t19047)
}
