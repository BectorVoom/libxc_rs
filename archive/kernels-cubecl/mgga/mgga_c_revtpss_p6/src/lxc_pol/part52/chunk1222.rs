//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1222/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1222<F: Float>(t100974: F, t100981: F, t100987: F, t121716: F, t121751: F, t127218: F, t127593: F, t127914: F, t127966: F, t1711: F, t1940: F, t25759: F, t26425: F, t27770: F, t27793: F, t27799: F, t27806: F, t28291: F, t28472: F, t32487: F, t32491: F, t32498: F, t34097: F, t7869: F) -> F {
    let t128097 = t1940 * t32487 * t1711 / F::cast_from(2.0_f64) + t28472 * t127218 + t28472 * t100974 * t34097 - F::cast_from(3.0_f64) * t28472 * t100981 * t127914 - F::cast_from(3.0_f64) * t28291 * t25759 * t127966 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t100987 * t32498 - t1940 * t32491 * t27806 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t121751 * t27793 - t1940 * t121716 * t7869 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t121751 * t27770 + t28472 * t27799 * t127593;
    t128097
}
