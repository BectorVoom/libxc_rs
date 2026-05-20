//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3788/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3788<F: Float>(t3566: F, t6695: F, t5497: F, t1811: F, t5216: F, t17288: F, t488: F, t5219: F, t1211: F, t1215: F, t1274: F, t1277: F, t1295: F, t17973: F, t17974: F, t17988: F, t18018: F, t18084: F, t1829: F, t3567: F, t3569: F, t3737: F, t3790: F, t5231: F, t5251: F, t56384: F, t56416: F, t6744: F, t69652: F, t70513: F, t72140: F, t72187: F, t72231: F, t72276: F, t72315: F, t72358: F, t72404: F, t72449: F, t72492: F, t72530: F, t72572: F, t72618: F, t72659: F, t72708: F, t72757: F) -> F {
    let t72767 = t3566 * t6695;
    let t72780 = t5497 * t5497;
    let t72784 = t5216 * t1811;
    let t72787 = t17288 * t1811;
    let t72794 = t5219 * t488;
    let t72797 = -F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1277 * (t69652 + t72140 + t72187 + t72231 + t72276 + t72315 + t72358 + t72404 + t72449 + t72492 + t72530 + t72572 + t72618 + t72659 + t72708 + t72757) - F::cast_from(0.13170898365871023197e1_f64) * t56384 * t1829 + F::cast_from(0.13170898365871023197e1_f64) * t72767 * t3569 + F::cast_from(0.13170898365871023197e1_f64) * t5251 * t18084 + F::cast_from(0.52683593463484092788e1_f64) * t56416 * t5231 - F::cast_from(0.52683593463484092788e1_f64) * t17973 * t17974 * t18018 + F::cast_from(0.26341796731742046394e1_f64) * t3567 * t1211 * t70513 + F::cast_from(0.26341796731742046394e1_f64) * t1274 * t3737 * t72780 - F::cast_from(0.26341796731742046394e1_f64) * t72784 * t1295 - F::cast_from(0.26341796731742046394e1_f64) * t72787 * t1215 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t3737 * t6744 * t3790 - F::cast_from(0.52683593463484092788e1_f64) * t72794 * t17988;
    t72797
}
