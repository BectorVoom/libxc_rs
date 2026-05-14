//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 526/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk526<F: Float>(t1089: F, t378: F, t4866: F, t3316: F, t342: F, t1043: F, t3302: F, t357: F, t4893: F, t1678: F, t359: F, t999: F, t380: F, t4930: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3223: F, t3278: F, t3287: F, t381: F, t4743: F, t4857: F, t4954: F, t4961: F, t4964: F, t4967: F, t4970: F, t4977: F, t4981: F, t4984: F, t4988: F, t989: F) -> (F, F) {
    let t4992 = t378 * t4866 * t1089;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t4997 = t3302 * t1043;
    let t4998 = t4997 * t357;
    let t4999 = t4893 * t4998;
    let t5004 = t359 * t1678;
    let t5005 = t5004 * t999;
    let t5009 = t1678 * t1043 * t1089;
    let t5012 = t380 * t4930;
    let t5015 = 0.65854491829355115987e0 * t4743 * t381 - 0.65854491829355115987e0 * t4857 * t1083 + 0.65854491829355115987e0 * t4954 * t1090 + 0.65854491829355115987e0 * t1647 * t1093 - 0.65854491829355115987e0 * t3223 * t1685 + 0.13170898365871023197e1 * t3204 * t4961 - 0.65854491829355115987e0 * t3287 * t4964 - 0.65854491829355115987e0 * t1024 * t4967 - 0.65854491829355115987e0 * t1024 * t4970 + 0.65854491829355115987e0 * t3278 * t1689 - 0.65854491829355115987e0 * t3287 * t4977 + 0.13170898365871023197e1 * t4981 * t4984 + 0.65854491829355115987e0 * t1087 * t4988 + 0.65854491829355115987e0 * t1087 * t4992 - 0.65854491829355115987e0 * t4996 * t4999 + 0.65854491829355115987e0 * t989 * t1692 - 0.65854491829355115987e0 * t1024 * t5005 + 0.65854491829355115987e0 * t1087 * t5009 + 0.65854491829355115987e0 * t342 * t5012;
    (t4998, t5015)
}
