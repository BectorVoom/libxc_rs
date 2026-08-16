//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2648/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2648(t1828: f64, t5245: f64, t1277: f64, t1774: f64, t5497: f64, t3736: f64, t5428: f64, t1204: f64, t1210: f64, t1770: f64, t1775: f64, t17986: f64, t18054: f64, t18062: f64, t18087: f64, t18114: f64, t1829: f64, t3556: f64, t3561: f64, t5220: f64, t5246: f64, t5251: f64, t5414: f64, t5423: f64, t6580: f64, t6588: f64, t6697: f64, t6703: f64) -> (f64, f64, f64, f64, f64) {
    let t21365 = t5245 * t1828;
    let t21366 = t1277 * t21365;
    let t21382 = t1277 * t1774 * t5497;
    let t21389 = t3736 * t1774;
    let t21390 = t21389 * t5428;
    let t21393 = -0.13170898365871023197e1_f64 * t18087 * t1829 + 0.13170898365871023197e1_f64 * t1770 * t5414 + 0.65854491829355115987e0_f64 * t1204 * t6697 + 0.13170898365871023197e1_f64 * t1210 * t21366 - 0.13170898365871023197e1_f64 * t18054 * t1829 - 0.13170898365871023197e1_f64 * t18114 * t1775 - 0.13170898365871023197e1_f64 * t18062 * t1775 + 0.13170898365871023197e1_f64 * t5220 * t5423 - 0.13170898365871023197e1_f64 * t5251 * t5246 - 0.65854491829355115987e0_f64 * t3556 * t6588 + 0.13170898365871023197e1_f64 * t1210 * t21382 + 0.13170898365871023197e1_f64 * t3556 * t6580 + 0.13170898365871023197e1_f64 * t3561 * t6703 - 0.26341796731742046394e1_f64 * t17986 * t21390;
    (t21366, t21382, t21389, t21390, t21393)
}
