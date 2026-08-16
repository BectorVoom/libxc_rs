//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2167/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2167<F: Float>(t1646: F, t1651: F, t100705: F, t107226: F, t1096: F, t1695: F, t19483: F, t1985: F, t20215: F, t25476: F, t25591: F, t25695: F, t25699: F, t27423: F, t27568: F, t27579: F, t27635: F, t27640: F, t27642: F, t27661: F, t27692: F, t29844: F, t29876: F, t29883: F, t4773: F, t5015: F, t6251: F, t7102: F, t7145: F, t7151: F, t7159: F, t7160: F, t7810: F, t93921: F, t94063: F, t94064: F, t94095: F, t988: F, t999: F, t99969: F) -> F {
    let t107532 = t1646 * t1651;
    let t107557 = F::cast_from(0.17347256376410398924e1_f64) * t7159 * t7160 * t7810 * t5015 - F::cast_from(0.17347256376410398924e1_f64) * t27661 * t27423 - F::cast_from(0.52041769129231196772e1_f64) * t100705 * t27692 + F::cast_from(0.13170898365871023197e1_f64) * t7102 * t20215 - F::cast_from(0.8673628188205199462e0_f64) * t25476 * t29876 - F::cast_from(0.8673628188205199462e0_f64) * t94063 * t107226 * t94064 + F::cast_from(0.8673628188205199462e0_f64) * t27640 * t27642 * t19483 - F::cast_from(0.13170898365871023197e1_f64) * t27568 * t4773 + F::cast_from(0.13170898365871023197e1_f64) * t25695 * t6251 - F::cast_from(0.10408353825846239354e2_f64) * t99969 * t1985 * t107532 * t999 - F::cast_from(0.69389025505641595696e1_f64) * t93921 * t1985 * t107532 * t1096 + F::cast_from(0.34694512752820797848e1_f64) * t94095 * t29844 + F::cast_from(0.34694512752820797848e1_f64) * t27661 * t27635 + F::cast_from(0.17347256376410398924e1_f64) * t25591 * t7145 * t29883 * t988 - F::cast_from(0.26020884564615598386e1_f64) * t25699 * t7145 * t29883 * t999 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27579 * t1695;
    t107557
}
