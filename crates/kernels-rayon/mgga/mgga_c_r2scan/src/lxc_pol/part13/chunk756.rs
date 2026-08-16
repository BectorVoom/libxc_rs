//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 756/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk756(t128: f64, t6189: f64, t6188: f64, t494: f64, t538: f64, t113: f64, t2: f64, t386: f64, t1567: f64, t774: f64, t255: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6190 = t6189 * t128;
    let t6191 = t6188 * t6190;
    let t6192 = t538 * t494;
    let t6194 = t113 * t2 * t386;
    let t6195 = t6192 * t6194;
    let t6196 = t6191 * t6195;
    let t6203 = t1567 * t774;
    let t6205 = t571 * t6203 * t255;
    (t6190, t6191, t6194, t6196, t6203, t6205)
}
