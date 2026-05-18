//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1219/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1219<F: Float>(t127892: F, t892: F, t34097: F, t775: F, t121716: F, t125985: F, t126018: F, t127593: F, t127596: F, t1940: F, t25207: F, t26425: F, t26585: F, t27383: F, t27387: F, t28472: F, t30: F, t32491: F, t33740: F, t34090: F, t7787: F, t92790: F, t98763: F) -> (F, F, F) {
    let t127893 = t127892 * t892;
    let t127907 = t34097 * t775;
    let t127912 = t28472 * t27383 * t127593 - F::new(3.0) / F::new(2.0) * t26425 * t25207 * t127596 - t1940 * t32491 * t27387 / F::new(2.0) - t1940 * t26585 * t33740 / F::new(2.0) + t1940 * t127893 * t30 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t26425 * t92790 * t34090 - t1940 * t121716 * t7787 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t26425 * t125985 + t28472 * t98763 * t34097 + F::new(3.0) * t26425 * t27383 * t127907 + t28472 * t126018;
    (t127893, t127907, t127912)
}
