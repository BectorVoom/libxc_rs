//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 820/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk820<F: Float>(t407: F, t6551: F, t1170: F, t151: F, t1530: F, t3833: F, t3835: F, t3842: F, t3843: F, t3846: F, t5305: F, t5318: F, t5327: F, t6529: F, t6532: F, t6536: F, t6538: F, t6541: F, t6544: F, t6547: F) -> (F, F) {
    let t6552 = t6551 * t407;
    let t6555 = -F::cast_from(0.65854491829355115987e0_f64) * t3833 - F::cast_from(0.13170898365871023197e1_f64) * t5305 + F::cast_from(0.13170898365871023197e1_f64) * t6529 - F::cast_from(0.13170898365871023197e1_f64) * t3835 + t5318 - F::cast_from(0.65854491829355115987e0_f64) * t1170 * t6532 + F::cast_from(0.65854491829355115987e0_f64) * t6536 - F::cast_from(0.13170898365871023197e1_f64) * t1170 * t6538 - F::cast_from(0.13170898365871023197e1_f64) * t1170 * t6541 - F::cast_from(0.65854491829355115987e0_f64) * t1170 * t6544 + F::cast_from(0.26341796731742046394e1_f64) * t1530 * t6547 + t5327 - t3842 + F::cast_from(0.65854491829355115987e0_f64) * t3843 + t3846 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t6552;
    (t6552, t6555)
}
