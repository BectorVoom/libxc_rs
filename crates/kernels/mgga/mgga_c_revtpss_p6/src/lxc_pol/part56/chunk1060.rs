//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1060/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1060<F: Float>(t121096: F, t32218: F, t32265: F, t3974: F, t121184: F, t7063: F, t32240: F, t119967: F, t121173: F, t121206: F, t121177: F, t3999: F, t8584: F) -> (F, F, F, F, F, F, F) {
    let t121219 = F::cast_from(0.19274729307122665472e-1_f64) * t32218 * t121096;
    let t121227 = t32265 * t3974;
    let t121228 = F::cast_from(0.19833242244827205771e-3_f64) * t121227;
    let t121229 = t7063 * t121184;
    let t121230 = t121229 * t32240;
    let t121232 = t119967 * t121173;
    let t121233 = t121232 * t121206;
    let t121235 = t121232 * t121177;
    let t121239 = t8584 * t3999;
    (t121219, t121228, t121230, t121232, t121233, t121235, t121239)
}
