//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2171/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2171<F: Float>(t100494: F, t1043: F, t107226: F, t1089: F, t19400: F, t19415: F, t19502: F, t25461: F, t25476: F, t25605: F, t25634: F, t25651: F, t27550: F, t27621: F, t27631: F, t27653: F, t29731: F, t29818: F, t29835: F, t29866: F, t29872: F, t29887: F, t4772: F, t4941: F, t6234: F, t6393: F, t7135: F, t7144: F, t7145: F, t7151: F, t7160: F, t7174: F, t7810: F, t7822: F, t93928: F, t94085: F, t99675: F, t999: F) -> F {
    let t107733 = -F::cast_from(0.34694512752820797848e1_f64) * t25461 * t29818 - F::cast_from(0.8673628188205199462e0_f64) * t29835 * t7174 + F::cast_from(0.17347256376410398924e1_f64) * t100494 * t7822 + F::cast_from(0.13170898365871023197e1_f64) * t27550 * t4941 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t7135 * t6234 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t29887 * t1043 * t1089 + F::cast_from(0.26341796731742046394e1_f64) * t25651 * t19400 - F::cast_from(0.26020884564615598386e1_f64) * t93928 * t29872 + F::cast_from(0.34694512752820797848e1_f64) * t25476 * t29866 + F::cast_from(0.13170898365871023197e1_f64) * t25651 * t19415 - F::cast_from(0.65854491829355115987e0_f64) * t25634 * t6393 - F::cast_from(0.8673628188205199462e0_f64) * t27621 * t27631 + F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7145 * t7810 * t4772 + F::cast_from(0.17347256376410398924e1_f64) * t94085 * t107226 * t19502 - F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7160 * t29731 * t999 - F::cast_from(0.17347256376410398924e1_f64) * t99675 * t27653;
    t107733
}
