//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3788/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3788(t3566: f64, t6695: f64, t5497: f64, t1811: f64, t5216: f64, t17288: f64, t488: f64, t5219: f64, t1211: f64, t1215: f64, t1274: f64, t1277: f64, t1295: f64, t17973: f64, t17974: f64, t17988: f64, t18018: f64, t18084: f64, t1829: f64, t3567: f64, t3569: f64, t3737: f64, t3790: f64, t5231: f64, t5251: f64, t56384: f64, t56416: f64, t6744: f64, t69652: f64, t70513: f64, t72140: f64, t72187: f64, t72231: f64, t72276: f64, t72315: f64, t72358: f64, t72404: f64, t72449: f64, t72492: f64, t72530: f64, t72572: f64, t72618: f64, t72659: f64, t72708: f64, t72757: f64) -> f64 {
    let t72767 = t3566 * t6695;
    let t72780 = t5497 * t5497;
    let t72784 = t5216 * t1811;
    let t72787 = t17288 * t1811;
    let t72794 = t5219 * t488;
    let t72797 = -0.65854491829355115987e0_f64 * t1274 * t1277 * (t69652 + t72140 + t72187 + t72231 + t72276 + t72315 + t72358 + t72404 + t72449 + t72492 + t72530 + t72572 + t72618 + t72659 + t72708 + t72757) - 0.13170898365871023197e1_f64 * t56384 * t1829 + 0.13170898365871023197e1_f64 * t72767 * t3569 + 0.13170898365871023197e1_f64 * t5251 * t18084 + 0.52683593463484092788e1_f64 * t56416 * t5231 - 0.52683593463484092788e1_f64 * t17973 * t17974 * t18018 + 0.26341796731742046394e1_f64 * t3567 * t1211 * t70513 + 0.26341796731742046394e1_f64 * t1274 * t3737 * t72780 - 0.26341796731742046394e1_f64 * t72784 * t1295 - 0.26341796731742046394e1_f64 * t72787 * t1215 + 0.13170898365871023197e1_f64 * t1274 * t3737 * t6744 * t3790 - 0.52683593463484092788e1_f64 * t72794 * t17988;
    t72797
}
