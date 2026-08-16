//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2199/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2199<F: Float>(t98678: F, t98725: F, t98776: F, t99563: F, t1096: F, t357: F, t1043: F, t1089: F, t16318: F, t16577: F, t25473: F, t25611: F, t25648: F, t25695: F, t27543: F, t27556: F, t27575: F, t27642: F, t27651: F, t27661: F, t27696: F, t3075: F, t3118: F, t3270: F, t3325: F, t4764: F, t4975: F, t4982: F, t4997: F, t7140: F, t7145: F, t7151: F, t7159: F, t7160: F, t7810: F, t93497: F, t93498: F, t94016: F, t94063: F, t94080: F, t94085: F, t94122: F, t988: F, t999: F) -> (F, F) {
    let t99565 = t98678 + t98725 + t98776 + t99563;
    let t99566 = t357 * t1096;
    let t99618 = -F::cast_from(0.17347256376410398924e1_f64) * t94063 * t27642 * t4997 * t99566 - F::cast_from(0.26020884564615598386e1_f64) * t94016 * t27651 * t4975 * t3270 - F::cast_from(0.34694512752820797848e1_f64) * t93497 * t27575 * t93498 + F::cast_from(0.17347256376410398924e1_f64) * t7159 * t7160 * t27543 * t1096 - F::cast_from(0.17347256376410398924e1_f64) * t27661 * t25648 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7160 * t7810 * t3325 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t27556 * t1043 * t1089 - F::cast_from(0.34694512752820797848e1_f64) * t94080 * t27642 * t4982 * t1043 * t988 + F::cast_from(0.34694512752820797848e1_f64) * t94085 * t27642 * t4982 * t3118 - F::cast_from(0.26020884564615598386e1_f64) * t94122 * t27651 * t16577 + F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7145 * t27543 * t999 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7145 * t7810 * t3075 + F::cast_from(0.13170898365871023197e1_f64) * t7140 * t16318 - F::cast_from(0.52041769129231196772e1_f64) * t25473 * t27696 + F::cast_from(0.13170898365871023197e1_f64) * t25695 * t4764;
    (t99565, t99618)
}
