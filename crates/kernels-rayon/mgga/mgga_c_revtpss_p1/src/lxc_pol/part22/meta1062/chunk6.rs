//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3798/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3798(t12654: f64, t17331: f64, t1775: f64, t17995: f64, t18019: f64, t18070: f64, t18087: f64, t18090: f64, t1813: f64, t20748: f64, t20756: f64, t21382: f64, t3556: f64, t3791: f64, t45427: f64, t5216: f64, t5220: f64, t5231: f64, t5251: f64, t5414: f64, t5429: f64, t56519: f64, t56607: f64, t60087: f64, t6745: f64) -> f64 {
    let t73109 = 0.26341796731742046394e1_f64 * t5216 * t5414 + 0.26341796731742046394e1_f64 * t5220 * t18019 - 0.26341796731742046394e1_f64 * t56519 * t1775 + 0.52683593463484092788e1_f64 * t17995 * t18070 + 0.13170898365871023197e1_f64 * t17331 * t1813 - 0.13170898365871023197e1_f64 * t20756 * t3791 - 0.65854491829355115987e0_f64 * t12654 * t6745 - 0.13170898365871023197e1_f64 * t60087 * t1775 - 0.79025390195226139182e1_f64 * t45427 * t20748 + 0.52683593463484092788e1_f64 * t18087 * t5429 + 0.26341796731742046394e1_f64 * t3556 * t21382 + 0.52683593463484092788e1_f64 * t56607 * t5231 - 0.13170898365871023197e1_f64 * t5251 * t18090;
    t73109
}
