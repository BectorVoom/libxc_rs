//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2170/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2170<F: Float>(t100708: F, t1089: F, t1646: F, t1647: F, t1652: F, t19396: F, t1978: F, t19856: F, t25473: F, t25634: F, t27437: F, t27543: F, t27545: F, t27604: F, t27639: F, t27643: F, t27647: F, t27665: F, t27668: F, t27670: F, t27699: F, t29752: F, t29807: F, t4743: F, t4866: F, t5016: F, t6351: F, t7102: F, t7144: F, t7145: F, t7151: F, t7167: F, t7812: F, t7825: F, t999: F, t99909: F, t99915: F) -> F {
    let t107691 = -F::cast_from(0.26020884564615598386e1_f64) * t25473 * t29752 + F::cast_from(0.13170898365871023197e1_f64) * t1647 * t27545 + F::cast_from(0.65854491829355115987e0_f64) * t19856 * t1978 + F::cast_from(0.17347256376410398924e1_f64) * t99909 * t27647 + F::cast_from(0.17347256376410398924e1_f64) * t99915 * t27437 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7145 * t29807 * t999 - F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7145 * t27543 * t1646 + F::cast_from(0.13170898365871023197e1_f64) * t25634 * t6351 + F::cast_from(0.13170898365871023197e1_f64) * t7102 * t19396 + F::cast_from(0.17347256376410398924e1_f64) * t99909 * t27665 - F::cast_from(0.17347256376410398924e1_f64) * t7825 * t27668 * t27670 + F::cast_from(0.8673628188205199462e0_f64) * t7825 * t27639 * t27643 - F::cast_from(0.13170898365871023197e1_f64) * t27699 * t5016 + F::cast_from(0.13170898365871023197e1_f64) * t4743 * t7812 - F::cast_from(0.13170898365871023197e1_f64) * t100708 * t1652 - F::cast_from(0.8673628188205199462e0_f64) * t7167 * t27604 * t4866 * t1089;
    t107691
}
