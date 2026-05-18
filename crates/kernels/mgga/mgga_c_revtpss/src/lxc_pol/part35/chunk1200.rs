//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1200/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1200<F: Float>(t102131: F, t102133: F, t102135: F, t102139: F, t102205: F, t109417: F, t109426: F, t109434: F, t109437: F, t115067: F, t1882: F, t213: F, t225: F, t30247: F, t543: F, t561: F, t6843: F, t7295: F, t7301: F, t8085: F, t96210: F, t96218: F, t96230: F) -> F {
    let t115098 = F::new(0.16463622957338778996e-1) * t109417 + F::new(0.72280234901709995519e-3) * t102131 + F::new(0.51405703062096148814e-2) * t102133 - F::new(0.68549505033305214441e-2) * t102135 - F::new(0.19514881078765566038e-2) * t102139 - F::new(0.21684070470512998656e-1) * t109426 - t96210 + F::new(0.13010442282307799193e1) * t7295 * t7301 * t8085 * t6843 * t543 - t96218 + F::new(0.15421710918628844643e0) * t109434 + F::new(0.13010442282307799193e1) * t7295 * t7301 * t30247 * t1882 * t543 - F::new(0.23132566377943266966e0) * t109437 + t96230 + F::new(0.65854491829355115987e0) * t213 * t115067 * t225 * t561 + F::new(0.13709901006661042888e-1) * t102205;
    t115098
}
