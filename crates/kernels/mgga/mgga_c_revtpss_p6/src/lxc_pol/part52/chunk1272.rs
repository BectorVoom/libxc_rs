//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1272/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1272<F: Float>(t28182: F, t8698: F, t34261: F, t7374: F, t32392: F, t7978: F, t32394: F, t28760: F, t8634: F, t34167: F, t649: F, t119578: F, t125948: F, t27123: F, t27126: F, t28588: F, t28727: F, t28935: F, t32410: F, t32621: F, t4248: F, t7732: F, t8568: F, t8637: F) -> F {
    let t128874 = t8698 * t28182;
    let t128876 = F::cast_from(2.0_f64) * t34261 * t7374;
    let t128878 = F::cast_from(2.0_f64) * t32392 * t7978;
    let t128880 = F::cast_from(2.0_f64) * t32394 * t7978;
    let t128882 = F::cast_from(2.0_f64) * t8634 * t28760;
    let t128891 = t649 * t34167;
    let t128897 = -F::cast_from(3.0_f64) * t119578 * t28588 - F::cast_from(2.0_f64) * t27123 * t8637 - F::cast_from(2.0_f64) * t27126 * t8637 - t28727 * t8568 + F::cast_from(3.0_f64) * t28935 * t8568 - F::cast_from(2.0_f64) * t32410 * t7732 - F::cast_from(2.0_f64) * t32621 * t4248 - t125948 - t128874 - t128876 - t128878 - t128880 - t128882 - t128891;
    t128897
}
