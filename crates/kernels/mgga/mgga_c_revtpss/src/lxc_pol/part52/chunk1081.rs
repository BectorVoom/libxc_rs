//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1081/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1081<F: Float>(t102854: F, t121716: F, t121793: F, t127582: F, t127892: F, t1583: F, t18875: F, t1940: F, t1962: F, t198: F, t207: F, t2403: F, t25445: F, t26425: F, t26585: F, t27363: F, t27384: F, t28460: F, t28472: F, t32491: F, t34080: F, t4343: F, t4537: F, t7086: F, t7432: F, t775: F, t7782: F, t8657: F, t890: F, t892: F, t92742: F) -> (F,) {
    let t128014 = t198 * t207 * t127892 * t892 + 6.0 * t26425 * t25445 * t18875 - t1940 * t26585 * t7782 - t1940 * t32491 * t4537 + 3.0 * t2403 * t34080 * t775 + 2.0 * t1940 * t121793 * t27384 - t1940 * t121716 * t1583 - t1940 * t28460 * t7086 - t1940 * t102854 * t1962 + 3.0 * t2403 * t8657 * t4343 - t1940 * t7432 * t27363 - 6.0 * t28472 * t92742 * t27384 - t1940 * t127582 * t890 - 3.0 * t2403 * t32491 * t18875;
    (t128014,)
}
