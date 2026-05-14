//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1083/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1083<F: Float>(t30: F, t265: F, t393: F, t128014: F, t128060: F, t127592: F, t127912: F, t127939: F, t127976: F, t1469: F, t32535: F, t34127: F, t4186: F, t45: F, t606: F, t8671: F, t100974: F, t100981: F, t100987: F, t121716: F, t121751: F, t127218: F, t127593: F, t127914: F, t127966: F, t1711: F, t1940: F, t25759: F, t26425: F, t27770: F, t27793: F, t27799: F, t27806: F, t28291: F, t28472: F, t32487: F, t32491: F, t32498: F, t34097: F, t7869: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t128061 = t128014 + t128060;
    let t128062 = piecewise3(t394, 0.0, t128061);
    let t128069 = piecewise3(t120, t127592 + t127912 + t127939 + t127976, t128062 * t45 / 2.0 + t32535 * t1469 / 2.0 + t34127 * t606 / 2.0 + t8671 * t4186 / 2.0);
    let t128097 = t1940 * t32487 * t1711 / 2.0 + t28472 * t127218 + t28472 * t100974 * t34097 - 3.0 * t28472 * t100981 * t127914 - 3.0 * t28291 * t25759 * t127966 - 3.0 / 2.0 * t26425 * t100987 * t32498 - t1940 * t32491 * t27806 / 2.0 - 3.0 / 2.0 * t121751 * t27793 - t1940 * t121716 * t7869 / 2.0 - 3.0 / 2.0 * t121751 * t27770 + t28472 * t27799 * t127593;
    (t128061, t128069, t128097)
}
