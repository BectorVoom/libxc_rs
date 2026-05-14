//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 691/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk691<F: Float>(t1655: F, t310: F, t119: F, t151: F, t3833: F, t3835: F, t3837: F, t3839: F, t3842: F, t3843: F, t3846: F, t4251: F, t5300: F, t5305: F, t5307: F, t5310: F, t5318: F, t5319: F, t5322: F) -> (F, F) {
    let t5327 = 0.13170898365871023197e1 * t310 * t1655;
    let t5329 = 0.13170898365871023197e1 * t151 * t4251 + 0.65854491829355115987e0 * t119 * t5300 - 0.13170898365871023197e1 * t3833 - 0.65854491829355115987e0 * t5305 - 0.65854491829355115987e0 * t151 * t5307 - 0.65854491829355115987e0 * t151 * t5310 - 0.26341796731742046395e1 * t3835 + 0.13170898365871023197e1 * t3837 + t5318 - 0.13170898365871023197e1 * t151 * t5319 - 0.65854491829355115987e0 * t151 * t5322 + 0.65854491829355115987e0 * t3839 + t5327 - t3842 + 0.13170898365871023197e1 * t3843 + t3846;
    (t5327, t5329)
}
