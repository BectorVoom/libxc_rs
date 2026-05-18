//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1348/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1348<F: Float>(t1828: F, t5245: F, t1277: F, t1774: F, t5497: F, t3736: F, t5428: F, t1204: F, t1210: F, t1770: F, t1775: F, t17986: F, t18054: F, t18062: F, t18087: F, t18114: F, t1829: F, t3556: F, t3561: F, t5220: F, t5246: F, t5251: F, t5414: F, t5423: F, t6580: F, t6588: F, t6697: F, t6703: F) -> F {
    let t21365 = t5245 * t1828;
    let t21366 = t1277 * t21365;
    let t21382 = t1277 * t1774 * t5497;
    let t21389 = t3736 * t1774;
    let t21390 = t21389 * t5428;
    let t21393 = -F::new(0.13170898365871023197e1) * t18087 * t1829 + F::new(0.13170898365871023197e1) * t1770 * t5414 + F::new(0.65854491829355115987e0) * t1204 * t6697 + F::new(0.13170898365871023197e1) * t1210 * t21366 - F::new(0.13170898365871023197e1) * t18054 * t1829 - F::new(0.13170898365871023197e1) * t18114 * t1775 - F::new(0.13170898365871023197e1) * t18062 * t1775 + F::new(0.13170898365871023197e1) * t5220 * t5423 - F::new(0.13170898365871023197e1) * t5251 * t5246 - F::new(0.65854491829355115987e0) * t3556 * t6588 + F::new(0.13170898365871023197e1) * t1210 * t21382 + F::new(0.13170898365871023197e1) * t3556 * t6580 + F::new(0.13170898365871023197e1) * t3561 * t6703 - F::new(0.26341796731742046394e1) * t17986 * t21390;
    t21393
}
