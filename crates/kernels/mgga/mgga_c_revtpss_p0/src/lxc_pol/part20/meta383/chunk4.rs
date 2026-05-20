//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1401/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1401<F: Float>(t10510: F, t11044: F, t675: F, t886: F, t10995: F, t268: F, t2828: F, t252: F, t257: F, t39644: F, t8779: F, t123: F, t2434: F, t2771: F) -> (F, F, F, F) {
    let t41038 = t11044 * t10510;
    let t41040 = t675 * t886;
    let t41043 = t10995 * t268 * t41040 * t2828;
    let t41049 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t252 * t257 * t8779 * t268;
    let t41052 = t10995 * t123 * t2434 * t2771;
    (t41038, t41043, t41049, t41052)
}
