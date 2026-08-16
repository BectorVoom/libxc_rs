//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 658/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk658(t1160: f64, t5316: f64, t1251: f64, t1411: f64, t2925: f64, t525: f64, t1655: f64, t310: f64, t119: f64, t151: f64, t3833: f64, t3835: f64, t3837: f64, t3839: f64, t3842: f64, t3843: f64, t3846: f64, t4251: f64, t5300: f64, t5305: f64, t5307: f64, t5310: f64) -> f64 {
    let t5318 = 0.13170898365871023197e1_f64 * t1160 * t5316;
    let t5319 = t1251 * t1411;
    let t5322 = t2925 * t525;
    let t5327 = 0.13170898365871023197e1_f64 * t310 * t1655;
    let t5329 = 0.13170898365871023197e1_f64 * t151 * t4251 + 0.65854491829355115987e0_f64 * t119 * t5300 - 0.13170898365871023197e1_f64 * t3833 - 0.65854491829355115987e0_f64 * t5305 - 0.65854491829355115987e0_f64 * t151 * t5307 - 0.65854491829355115987e0_f64 * t151 * t5310 - 0.26341796731742046395e1_f64 * t3835 + 0.13170898365871023197e1_f64 * t3837 + t5318 - 0.13170898365871023197e1_f64 * t151 * t5319 - 0.65854491829355115987e0_f64 * t151 * t5322 + 0.65854491829355115987e0_f64 * t3839 + t5327 - t3842 + 0.13170898365871023197e1_f64 * t3843 + t3846;
    t5329
}
