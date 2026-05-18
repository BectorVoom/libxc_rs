//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 591/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk591<F: Float>(t378: F, t6305: F, t3304: F, t1089: F, t1668: F, t1678: F, t6299: F, t3318: F, t380: F, t6343: F, t1024: F, t1087: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3287: F, t3299: F, t3317: F, t342: F, t381: F, t4857: F, t4954: F, t6235: F, t6362: F, t6365: F, t6368: F, t6371: F) -> (F, F, F, F, F, F) {
    let t6374 = t378 * t6305;
    let t6375 = t6374 * t3304;
    let t6379 = t1678 * t1668 * t1089;
    let t6383 = t378 * t6299 * t1089;
    let t6386 = t6374 * t3318;
    let t6389 = t380 * t6343;
    let t6392 = F::new(0.65854491829355115987e0) * t6235 * t381 - F::new(0.13170898365871023197e1) * t4857 * t1685 + F::new(0.13170898365871023197e1) * t4954 * t1689 + F::new(0.13170898365871023197e1) * t1647 * t1692 + F::new(0.13170898365871023197e1) * t3204 * t6362 - F::new(0.13170898365871023197e1) * t3287 * t6365 - F::new(0.13170898365871023197e1) * t1024 * t6368 - F::new(0.65854491829355115987e0) * t1024 * t6371 + F::new(0.13170898365871023197e1) * t3299 * t6375 + F::new(0.13170898365871023197e1) * t1087 * t6379 + F::new(0.65854491829355115987e0) * t1087 * t6383 - F::new(0.65854491829355115987e0) * t3317 * t6386 + F::new(0.65854491829355115987e0) * t342 * t6389;
    (t6375, t6379, t6383, t6386, t6389, t6392)
}
