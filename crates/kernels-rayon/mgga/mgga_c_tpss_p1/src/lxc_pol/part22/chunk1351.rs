//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1351/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1351(t66379: f64, t66494: f64, t66546: f64, t66601: f64, t823: f64, t198: f64, t5864: f64, t20526: f64, t64302: f64, t1692: f64, t17931: f64, t18728: f64, t18807: f64, t19672: f64, t19819: f64, t19829: f64, t19836: f64, t20417: f64, t20510: f64, t2439: f64, t30: f64, t580: f64, t5849: f64, t63841: f64, t63847: f64, t63850: f64, t63864: f64, t64241: f64, t64260: f64, t64263: f64, t66311: f64, t66317: f64) -> (f64, f64, f64, f64, f64) {
    let t66603 = t66379 + t66494 + t66546 + t66601;
    let t66604 = t66603 * t823;
    let t66608 = t198 * t5864;
    let t66615 = 2.0_f64 * t20526 * t64302;
    let t66618 = 6.0_f64 * t20417 * t64260 + 6.0_f64 * t20417 * t64263 + 3.0_f64 * t20417 * t64241 + 6.0_f64 * t66311 * t19672 - t1692 * t18807 * t19836 - 3.0_f64 * t66317 * t17931 - 3.0_f64 * t20417 * t63864 + t1692 * t20510 * t580 - 3.0_f64 * t18728 * t63847 - 3.0_f64 * t18728 * t63850 + t1692 * t66604 * t30 / 2.0_f64 + 2.0_f64 * t66608 * t19819 + 3.0_f64 * t2439 * t5849 * t19829 - t66615 - 3.0_f64 * t18728 * t63841;
    (t66603, t66604, t66608, t66615, t66618)
}
