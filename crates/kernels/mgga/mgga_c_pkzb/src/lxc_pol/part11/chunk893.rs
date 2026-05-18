//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 893/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk893<F: Float>(t790: F, t9712: F, t1134: F, t1144: F, t2957: F, t2965: F, t2990: F, t307: F, t311: F, t3670: F, t3676: F, t3695: F, t786: F, t800: F, t9634: F, t9648: F, t9651: F, t9657: F) -> (F, F) {
    let t9713 = t790 * t9712;
    let t9716 = F::new(0.65854491829355115987e0) * t9634 * t311 - F::new(0.65854491829355115987e0) * t3670 * t800 - F::new(0.13170898365871023197e1) * t2957 * t1144 + F::new(0.26341796731742046394e1) * t1134 * t2965 - F::new(0.13170898365871023197e1) * t1134 * t2990 + F::new(0.13170898365871023197e1) * t786 * t3676 - F::new(0.39512695097613069591e1) * t307 * t9648 + F::new(0.26341796731742046394e1) * t307 * t9651 - F::new(0.65854491829355115987e0) * t786 * t3695 + F::new(0.13170898365871023197e1) * t307 * t9657 - F::new(0.65854491829355115987e0) * t307 * t9713;
    (t9713, t9716)
}
