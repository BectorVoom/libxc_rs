//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1624/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1624<F: Float>(t1210: F, t1274: F, t1770: F, t1775: F, t1813: F, t1829: F, t3567: F, t460: F, t495: F, t5220: F, t5225: F, t5251: F, t5417: F, t6564: F, t6574: F, t6580: F, t6588: F, t6697: F, t6703: F, t6745: F) -> F {
    let t6748 = F::cast_from(0.65854491829355115987e0_f64) * t6564 * t495 - F::cast_from(0.13170898365871023197e1_f64) * t5220 * t1775 + F::cast_from(0.13170898365871023197e1_f64) * t1770 * t1813 - F::cast_from(0.13170898365871023197e1_f64) * t5225 * t1829 + F::cast_from(0.13170898365871023197e1_f64) * t3567 * t6574 - F::cast_from(0.13170898365871023197e1_f64) * t5251 * t1775 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t6580 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t6588 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t6697 - F::cast_from(0.13170898365871023197e1_f64) * t5417 * t1829 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t6703 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t6745;
    t6748
}
