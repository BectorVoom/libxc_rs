//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 726/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk726<F: Float>(t1160: F, t5316: F, t1251: F, t1411: F, t2925: F, t525: F, t1655: F, t310: F, t119: F, t151: F, t3833: F, t3835: F, t3837: F, t3839: F, t3842: F, t3843: F, t3846: F, t4251: F, t5300: F, t5305: F, t5307: F, t5310: F) -> (F, F, F, F, F) {
    let t5318 = F::cast_from(0.13170898365871023197e1_f64) * t1160 * t5316;
    let t5319 = t1251 * t1411;
    let t5322 = t2925 * t525;
    let t5327 = F::cast_from(0.13170898365871023197e1_f64) * t310 * t1655;
    let t5329 = F::cast_from(0.13170898365871023197e1_f64) * t151 * t4251 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t5300 - F::cast_from(0.13170898365871023197e1_f64) * t3833 - F::cast_from(0.65854491829355115987e0_f64) * t5305 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t5307 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t5310 - F::cast_from(0.26341796731742046395e1_f64) * t3835 + F::cast_from(0.13170898365871023197e1_f64) * t3837 + t5318 - F::cast_from(0.13170898365871023197e1_f64) * t151 * t5319 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t5322 + F::cast_from(0.65854491829355115987e0_f64) * t3839 + t5327 - t3842 + F::cast_from(0.13170898365871023197e1_f64) * t3843 + t3846;
    (t5318, t5319, t5322, t5327, t5329)
}
