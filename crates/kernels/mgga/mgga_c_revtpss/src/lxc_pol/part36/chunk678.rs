//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 678/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk678<F: Float>(t1210: F, t1274: F, t1770: F, t1775: F, t1813: F, t1829: F, t3567: F, t460: F, t495: F, t5220: F, t5225: F, t5251: F, t5417: F, t6564: F, t6574: F, t6580: F, t6588: F, t6697: F, t6703: F, t6745: F) -> F {
    let t6748 = F::new(0.65854491829355115987e0) * t6564 * t495 - F::new(0.13170898365871023197e1) * t5220 * t1775 + F::new(0.13170898365871023197e1) * t1770 * t1813 - F::new(0.13170898365871023197e1) * t5225 * t1829 + F::new(0.13170898365871023197e1) * t3567 * t6574 - F::new(0.13170898365871023197e1) * t5251 * t1775 + F::new(0.13170898365871023197e1) * t1210 * t6580 - F::new(0.65854491829355115987e0) * t1210 * t6588 + F::new(0.65854491829355115987e0) * t460 * t6697 - F::new(0.13170898365871023197e1) * t5417 * t1829 + F::new(0.13170898365871023197e1) * t1274 * t6703 - F::new(0.65854491829355115987e0) * t1274 * t6745;
    t6748
}
