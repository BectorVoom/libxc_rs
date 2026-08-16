//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2199/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2199(t98678: f64, t98725: f64, t98776: f64, t99563: f64, t1096: f64, t357: f64, t1043: f64, t1089: f64, t16318: f64, t16577: f64, t25473: f64, t25611: f64, t25648: f64, t25695: f64, t27543: f64, t27556: f64, t27575: f64, t27642: f64, t27651: f64, t27661: f64, t27696: f64, t3075: f64, t3118: f64, t3270: f64, t3325: f64, t4764: f64, t4975: f64, t4982: f64, t4997: f64, t7140: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7810: f64, t93497: f64, t93498: f64, t94016: f64, t94063: f64, t94080: f64, t94085: f64, t94122: f64, t988: f64, t999: f64) -> (f64, f64) {
    let t99565 = t98678 + t98725 + t98776 + t99563;
    let t99566 = t357 * t1096;
    let t99618 = -0.17347256376410398924e1_f64 * t94063 * t27642 * t4997 * t99566 - 0.26020884564615598386e1_f64 * t94016 * t27651 * t4975 * t3270 - 0.34694512752820797848e1_f64 * t93497 * t27575 * t93498 + 0.17347256376410398924e1_f64 * t7159 * t7160 * t27543 * t1096 - 0.17347256376410398924e1_f64 * t27661 * t25648 + 0.8673628188205199462e0_f64 * t7159 * t7160 * t7810 * t3325 + 0.17347256376410398924e1_f64 * t25611 * t27556 * t1043 * t1089 - 0.34694512752820797848e1_f64 * t94080 * t27642 * t4982 * t1043 * t988 + 0.34694512752820797848e1_f64 * t94085 * t27642 * t4982 * t3118 - 0.26020884564615598386e1_f64 * t94122 * t27651 * t16577 + 0.17347256376410398924e1_f64 * t7151 * t7145 * t27543 * t999 + 0.8673628188205199462e0_f64 * t7151 * t7145 * t7810 * t3075 + 0.13170898365871023197e1_f64 * t7140 * t16318 - 0.52041769129231196772e1_f64 * t25473 * t27696 + 0.13170898365871023197e1_f64 * t25695 * t4764;
    (t99565, t99618)
}
