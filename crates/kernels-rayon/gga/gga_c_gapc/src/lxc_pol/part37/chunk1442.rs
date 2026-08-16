//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1442/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1442(t224: f64, t38816: f64, t38825: f64, t38831: f64, t38835: f64, t12589: f64, t12623: f64, t12588: f64, t12654: f64, t987: f64, t36095: f64, t36100: f64, t36103: f64, t36105: f64, t36109: f64, t36111: f64, t36113: f64, t36116: f64, t36119: f64, t36270: f64, t36271: f64, t36275: f64, t36283: f64, t36285: f64, t38537: f64, t38556: f64, t38689: f64, t38692: f64) -> (f64, f64, f64, f64, f64) {
    let t38838 = t224 * (t38816 + t38825 + t38831 + t38835);
    let t38842 = 2.0_f64 * t12589;
    let t38843 = 2.0_f64 * t12623;
    let t38844 = 2.0_f64 * t12588;
    let t38863 = t987 * t12654;
    let t38891 = -t36095 + t38537 - t36100 - t36103 + t36105 + t38556 - t38689 - t36109 + 2.0_f64 * t38863 + t36111 - t36113 - t36116 + t36119 + t38692 - t36270 - t36271 - t36275 + t36283 - t36285;
    (t38838, t38842, t38843, t38844, t38891)
}
