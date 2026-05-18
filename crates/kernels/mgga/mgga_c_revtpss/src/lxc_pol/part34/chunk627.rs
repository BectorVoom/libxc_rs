//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 627/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk627<F: Float>(t1076: F, t1647: F, t1652: F, t1680: F, t1696: F, t3058: F, t342: F, t386: F, t4747: F, t4752: F, t4778: F, t4935: F, t6235: F, t6245: F, t6251: F, t6259: F, t6345: F, t6351: F, t6393: F, t995: F) -> F {
    let t6396 = F::new(0.65854491829355115987e0) * t6235 * t386 - F::new(0.13170898365871023197e1) * t4747 * t1652 + F::new(0.13170898365871023197e1) * t1647 * t1680 - F::new(0.13170898365871023197e1) * t4752 * t1696 + F::new(0.13170898365871023197e1) * t3058 * t6245 - F::new(0.13170898365871023197e1) * t4778 * t1652 + F::new(0.13170898365871023197e1) * t995 * t6251 - F::new(0.65854491829355115987e0) * t995 * t6259 + F::new(0.65854491829355115987e0) * t342 * t6345 - F::new(0.13170898365871023197e1) * t4935 * t1696 + F::new(0.13170898365871023197e1) * t1076 * t6351 - F::new(0.65854491829355115987e0) * t1076 * t6393;
    t6396
}
