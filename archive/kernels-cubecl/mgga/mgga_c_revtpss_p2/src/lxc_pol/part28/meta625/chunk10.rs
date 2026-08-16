//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2233/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2233<F: Float>(t1096: F, t16243: F, t16352: F, t1678: F, t1986: F, t25476: F, t25591: F, t25621: F, t25625: F, t25658: F, t25699: F, t27426: F, t27599: F, t27616: F, t27661: F, t27676: F, t27680: F, t27687: F, t3042: F, t3043: F, t3271: F, t5016: F, t7102: F, t7145: F, t7151: F, t7156: F, t7160: F, t7812: F, t7821: F, t94095: F, t988: F, t999: F) -> F {
    let t100650 = -F::cast_from(0.8673628188205199462e0_f64) * t27661 * t25621 + F::cast_from(0.65854491829355115987e0_f64) * t7102 * t16352 + F::cast_from(0.13170898365871023197e1_f64) * t27616 * t3271 + F::cast_from(0.13170898365871023197e1_f64) * t7102 * t16243 + F::cast_from(0.34694512752820797848e1_f64) * t94095 * t27599 - F::cast_from(0.8673628188205199462e0_f64) * t7156 * t27676 - F::cast_from(0.52041769129231196772e1_f64) * t25699 * t7145 * t27687 * t999 + F::cast_from(0.17347256376410398924e1_f64) * t25591 * t7145 * t7821 * t3042 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27687 * t988 - F::cast_from(0.13170898365871023197e1_f64) * t25658 * t5016 - F::cast_from(0.8673628188205199462e0_f64) * t25625 * t1678 * t1986 - F::cast_from(0.17347256376410398924e1_f64) * t25476 * t27680 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27687 * t1096 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27426 * t999 + F::cast_from(0.65854491829355115987e0_f64) * t3043 * t7812;
    t100650
}
