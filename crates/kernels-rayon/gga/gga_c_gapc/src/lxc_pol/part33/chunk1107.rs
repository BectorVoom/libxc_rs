//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1107/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1107(t10043: f64, t11945: f64, t11387: f64, t3363: f64, t1089: f64, t29228: f64, t3784: f64, t11944: f64, t2200: f64, t9896: f64, t18856: f64, t2767: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33755 = t10043 * t11945;
    let t33757 = t3363 * t11387;
    let t33758 = t33757 * t1089;
    let t33760 = t3784 * t29228;
    let t33763 = t11944 * t2200 * t9896;
    let t33766 = t18856 * t3717 * t2767;
    (t33755, t33757, t33758, t33760, t33763, t33766)
}
