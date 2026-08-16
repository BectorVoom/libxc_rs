//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3798/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3798<F: Float>(t12654: F, t17331: F, t1775: F, t17995: F, t18019: F, t18070: F, t18087: F, t18090: F, t1813: F, t20748: F, t20756: F, t21382: F, t3556: F, t3791: F, t45427: F, t5216: F, t5220: F, t5231: F, t5251: F, t5414: F, t5429: F, t56519: F, t56607: F, t60087: F, t6745: F) -> F {
    let t73109 = F::cast_from(0.26341796731742046394e1_f64) * t5216 * t5414 + F::cast_from(0.26341796731742046394e1_f64) * t5220 * t18019 - F::cast_from(0.26341796731742046394e1_f64) * t56519 * t1775 + F::cast_from(0.52683593463484092788e1_f64) * t17995 * t18070 + F::cast_from(0.13170898365871023197e1_f64) * t17331 * t1813 - F::cast_from(0.13170898365871023197e1_f64) * t20756 * t3791 - F::cast_from(0.65854491829355115987e0_f64) * t12654 * t6745 - F::cast_from(0.13170898365871023197e1_f64) * t60087 * t1775 - F::cast_from(0.79025390195226139182e1_f64) * t45427 * t20748 + F::cast_from(0.52683593463484092788e1_f64) * t18087 * t5429 + F::cast_from(0.26341796731742046394e1_f64) * t3556 * t21382 + F::cast_from(0.52683593463484092788e1_f64) * t56607 * t5231 - F::cast_from(0.13170898365871023197e1_f64) * t5251 * t18090;
    t73109
}
