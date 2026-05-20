//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2390/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2390<F: Float>(t1234: F, t12699: F, t12709: F, t12717: F, t12723: F, t1285: F, t17331: F, t1770: F, t17917: F, t17921: F, t17934: F, t17941: F, t17945: F, t17949: F, t17952: F, t17955: F, t17958: F, t1822: F, t3670: F, t3746: F, t3756: F, t3770: F, t3774: F, t3778: F, t3787: F, t490: F, t5436: F, t5446: F, t5466: F, t5470: F, t5491: F) -> F {
    let t17961 = F::cast_from(0.65854491829355115987e0_f64) * t12699 * t1822 + F::cast_from(0.65854491829355115987e0_f64) * t5436 * t3778 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t17917 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t17921 - F::cast_from(0.13170898365871023197e1_f64) * t12709 * t5446 - F::cast_from(0.13170898365871023197e1_f64) * t12723 * t5446 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t5491 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t5470 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t3787 + F::cast_from(0.13170898365871023197e1_f64) * t17934 * t3770 + F::cast_from(0.65854491829355115987e0_f64) * t17331 * t490 + F::cast_from(0.13170898365871023197e1_f64) * t5436 * t3774 - F::cast_from(0.13170898365871023197e1_f64) * t1234 * t17941 + F::cast_from(0.13170898365871023197e1_f64) * t12717 * t17945 + F::cast_from(0.65854491829355115987e0_f64) * t17949 * t17952 + F::cast_from(0.26341796731742046394e1_f64) * t17955 * t5466 - F::cast_from(0.13170898365871023197e1_f64) * t17958 * t3756;
    t17961
}
