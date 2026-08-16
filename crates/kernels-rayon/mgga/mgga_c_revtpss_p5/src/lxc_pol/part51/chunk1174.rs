//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1174/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1174(t3336: f64, t33836: f64, t1100: f64, t1102: f64, t120745: f64, t120749: f64, t120767: f64, t126471: f64, t126508: f64, t126546: f64, t126596: f64, t126640: f64, t126673: f64, t126708: f64, t126741: f64, t126786: f64, t126828: f64, t126868: f64, t126915: f64, t126948: f64, t126995: f64, t127035: f64, t127074: f64, t1699: f64, t198: f64, t25709: f64, t25713: f64, t27708: f64, t27717: f64, t32030: f64, t32036: f64, t336: f64, t5019: f64, t5023: f64, t7177: f64, t7181: f64, t7840: f64) -> f64 {
    let t127082 = t33836 * t3336;
    let t127112 = t198 * t336 * (t126471 + t126508 + t126546 + t126596 + t126640 + t126673 + t126708 + t126741 + t126786 + t126828 + t126868 + t126915 + t126948 + t126995 + t127035 + t127074) * t1102 - t5023 * t127082 * t1100 - t5023 * t120745 * t1699 + 2.0_f64 * t5023 * t120749 * t27717 - t5023 * t32030 * t5019 - 2.0_f64 * t5023 * t25709 * t7840 + 4.0_f64 * t5023 * t25713 * t7840 * t1100 - 2.0_f64 * t5023 * t7181 * t27708 + 4.0_f64 * t5023 * t25713 * t1699 * t7177 - 6.0_f64 * t5023 * t120767 * t27717 + 2.0_f64 * t5023 * t32036 * t5019;
    t127112
}
