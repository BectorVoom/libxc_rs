//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1304/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1304<F: Float>(t113103: F, t25759: F, t113432: F, t1711: F, t5962: F, t5966: F, t6079: F, t23279: F, t27763: F, t6075: F, t106516: F, t114101: F, t114104: F, t1940: F, t1963: F, t2403: F, t25206: F, t25445: F, t27158: F, t27368: F, t29705: F, t29946: F, t29949: F, t29967: F, t4541: F, t6416: F, t7091: F, t7783: F, t7869: F, t98637: F) -> F {
    let t114107 = t25759 * t113103;
    let t114110 = t25759 * t113432;
    let t114113 = t1711 * t5962;
    let t114117 = t1711 * t5966;
    let t114121 = t1711 * t6079;
    let t114128 = t27763 * t23279;
    let t114140 = t1711 * t6075;
    let t114149 = F::new(9.0) * t25206 * t114101 - F::new(9.0) * t25206 * t114104 - F::new(9.0) / F::new(2.0) * t25206 * t114107 - F::new(9.0) / F::new(2.0) * t25206 * t114110 + F::new(9.0) / F::new(2.0) * t2403 * t1963 * t114113 + F::new(9.0) * t4541 * t1963 * t114117 + F::new(3.0) * t1940 * t25445 * t114121 - F::new(3.0) * t1940 * t27368 * t29967 + F::new(9.0) * t27158 * t114128 + F::new(3.0) / F::new(2.0) * t1940 * t29705 * t1711 + F::new(3.0) / F::new(2.0) * t1940 * t7783 * t6416 - F::new(3.0) / F::new(2.0) * t1940 * t106516 * t7869 - F::new(3.0) / F::new(2.0) * t1940 * t7091 * t114140 - F::new(9.0) * t98637 * t29946 + F::new(9.0) * t2403 * t7783 * t29949;
    t114149
}
