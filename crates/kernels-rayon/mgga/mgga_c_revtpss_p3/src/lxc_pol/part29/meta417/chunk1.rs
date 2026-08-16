//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1533/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1533(t16573: f64, t4893: f64, t3059: f64, t4975: f64, t4781: f64, t12132: f64, t1647: f64, t3316: f64, t1083: f64, t12122: f64, t12127: f64, t12146: f64, t12149: f64, t12154: f64, t15655: f64, t16529: f64, t16534: f64, t16537: f64, t16540: f64, t16544: f64, t16552: f64, t16555: f64, t16559: f64, t16562: f64, t16566: f64, t16569: f64, t3278: f64, t3288: f64, t3309: f64, t3319: f64, t342: f64, t4954: f64, t4964: f64, t4977: f64, t4981: f64, t4996: f64, t5009: f64) -> f64 {
    let t16574 = t4893 * t16573;
    let t16577 = t4975 * t3059;
    let t16578 = t4781 * t16577;
    let t16581 = t4893 * t12132;
    let t16584 = t1647 * t3316;
    let t16589 = -0.13170898365871023197e1_f64 * t12154 * t4977 + 0.65854491829355115987e0_f64 * t342 * t16529 - 0.13170898365871023197e1_f64 * t15655 * t1083 - 0.13170898365871023197e1_f64 * t4996 * t16534 - 0.13170898365871023197e1_f64 * t12122 * t16537 + 0.65854491829355115987e0_f64 * t12127 * t16540 - 0.13170898365871023197e1_f64 * t16544 * t3288 - 0.13170898365871023197e1_f64 * t12146 * t4964 + 0.13170898365871023197e1_f64 * t3278 * t5009 + 0.39512695097613069591e1_f64 * t16552 * t16555 - 0.39512695097613069591e1_f64 * t16559 * t16562 + 0.65854491829355115987e0_f64 * t16566 * t16569 - 0.65854491829355115987e0_f64 * t4996 * t16574 + 0.13170898365871023197e1_f64 * t12149 * t16578 + 0.13170898365871023197e1_f64 * t4981 * t16581 - 0.65854491829355115987e0_f64 * t16584 * t3319 + 0.13170898365871023197e1_f64 * t4954 * t3309;
    t16589
}
