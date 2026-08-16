//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 661/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk661(t407: f64, t6551: f64, t1170: f64, t151: f64, t1530: f64, t3833: f64, t3835: f64, t3842: f64, t3843: f64, t3846: f64, t5305: f64, t5318: f64, t5327: f64, t6529: f64, t6532: f64, t6536: f64, t6538: f64, t6541: f64, t6544: f64, t6547: f64) -> f64 {
    let t6552 = t6551 * t407;
    let t6555 = -0.65854491829355115987e0_f64 * t3833 - 0.13170898365871023197e1_f64 * t5305 + 0.13170898365871023197e1_f64 * t6529 - 0.13170898365871023197e1_f64 * t3835 + t5318 - 0.65854491829355115987e0_f64 * t1170 * t6532 + 0.65854491829355115987e0_f64 * t6536 - 0.13170898365871023197e1_f64 * t1170 * t6538 - 0.13170898365871023197e1_f64 * t1170 * t6541 - 0.65854491829355115987e0_f64 * t1170 * t6544 + 0.26341796731742046394e1_f64 * t1530 * t6547 + t5327 - t3842 + 0.65854491829355115987e0_f64 * t3843 + t3846 - 0.65854491829355115987e0_f64 * t151 * t6552;
    t6555
}
