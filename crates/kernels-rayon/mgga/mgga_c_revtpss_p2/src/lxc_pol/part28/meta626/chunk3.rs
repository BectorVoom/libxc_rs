//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2238/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2238(t27708: f64, t3336: f64, t11108: f64, t7840: f64, t100425: f64, t100471: f64, t100513: f64, t100560: f64, t100606: f64, t100650: f64, t100696: f64, t100748: f64, t100794: f64, t1100: f64, t1102: f64, t16612: f64, t1699: f64, t198: f64, t25709: f64, t25713: f64, t27712: f64, t27717: f64, t3329: f64, t3333: f64, t336: f64, t5019: f64, t5023: f64, t63827: f64, t7181: f64, t94138: f64, t94142: f64, t94149: f64, t99618: f64, t99673: f64, t99728: f64, t99790: f64, t99847: f64, t99901: f64, t99950: f64) -> f64 {
    let t100802 = t27708 * t3336;
    let t100806 = t7840 * t11108;
    let t100833 = t198 * t336 * (t99618 + t99673 + t99728 + t99790 + t99847 + t99901 + t99950 + t100425 + t100471 + t100513 + t100560 + t100606 + t100650 + t100696 + t100748 + t100794) * t1102 - 2.0_f64 * t5023 * t100802 * t1100 + 2.0_f64 * t5023 * t100806 * t3333 - t5023 * t27712 * t3329 - t5023 * t94138 * t1699 + 4.0_f64 * t5023 * t94142 * t27717 - 2.0_f64 * t5023 * t25709 * t5019 - 6.0_f64 * t5023 * t94149 * t1699 * t3333 + 4.0_f64 * t5023 * t25713 * t63827 + 2.0_f64 * t5023 * t25713 * t1699 * t3329 - t5023 * t7181 * t16612;
    t100833
}
