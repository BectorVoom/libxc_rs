//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2243/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2243<F: Float>(t1083: F, t12122: F, t12127: F, t12146: F, t12149: F, t12154: F, t15655: F, t16529: F, t16534: F, t16537: F, t16540: F, t16544: F, t16552: F, t16555: F, t16559: F, t16562: F, t16566: F, t16569: F, t16574: F, t16578: F, t16581: F, t16584: F, t3278: F, t3288: F, t3309: F, t3319: F, t342: F, t4954: F, t4964: F, t4977: F, t4981: F, t4996: F, t5009: F) -> F {
    let t16589 = -F::cast_from(0.13170898365871023197e1_f64) * t12154 * t4977 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t16529 - F::cast_from(0.13170898365871023197e1_f64) * t15655 * t1083 - F::cast_from(0.13170898365871023197e1_f64) * t4996 * t16534 - F::cast_from(0.13170898365871023197e1_f64) * t12122 * t16537 + F::cast_from(0.65854491829355115987e0_f64) * t12127 * t16540 - F::cast_from(0.13170898365871023197e1_f64) * t16544 * t3288 - F::cast_from(0.13170898365871023197e1_f64) * t12146 * t4964 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t5009 + F::cast_from(0.39512695097613069591e1_f64) * t16552 * t16555 - F::cast_from(0.39512695097613069591e1_f64) * t16559 * t16562 + F::cast_from(0.65854491829355115987e0_f64) * t16566 * t16569 - F::cast_from(0.65854491829355115987e0_f64) * t4996 * t16574 + F::cast_from(0.13170898365871023197e1_f64) * t12149 * t16578 + F::cast_from(0.13170898365871023197e1_f64) * t4981 * t16581 - F::cast_from(0.65854491829355115987e0_f64) * t16584 * t3319 + F::cast_from(0.13170898365871023197e1_f64) * t4954 * t3309;
    t16589
}
