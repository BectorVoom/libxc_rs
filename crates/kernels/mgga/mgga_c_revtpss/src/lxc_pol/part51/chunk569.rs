//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 569/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk569<F: Float>(t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3223: F, t3278: F, t3287: F, t342: F, t381: F, t4743: F, t4857: F, t4954: F, t4961: F, t4964: F, t4967: F, t4970: F, t4977: F, t4981: F, t4984: F, t4988: F, t4992: F, t4996: F, t4999: F, t5005: F, t5009: F, t5012: F, t989: F) -> F {
    let t5015 = F::new(0.65854491829355115987e0) * t4743 * t381 - F::new(0.65854491829355115987e0) * t4857 * t1083 + F::new(0.65854491829355115987e0) * t4954 * t1090 + F::new(0.65854491829355115987e0) * t1647 * t1093 - F::new(0.65854491829355115987e0) * t3223 * t1685 + F::new(0.13170898365871023197e1) * t3204 * t4961 - F::new(0.65854491829355115987e0) * t3287 * t4964 - F::new(0.65854491829355115987e0) * t1024 * t4967 - F::new(0.65854491829355115987e0) * t1024 * t4970 + F::new(0.65854491829355115987e0) * t3278 * t1689 - F::new(0.65854491829355115987e0) * t3287 * t4977 + F::new(0.13170898365871023197e1) * t4981 * t4984 + F::new(0.65854491829355115987e0) * t1087 * t4988 + F::new(0.65854491829355115987e0) * t1087 * t4992 - F::new(0.65854491829355115987e0) * t4996 * t4999 + F::new(0.65854491829355115987e0) * t989 * t1692 - F::new(0.65854491829355115987e0) * t1024 * t5005 + F::new(0.65854491829355115987e0) * t1087 * t5009 + F::new(0.65854491829355115987e0) * t342 * t5012;
    t5015
}
