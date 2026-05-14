//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1034/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1034<F: Float>(t18875: F, t27799: F, t126017: F, t1113: F, t119706: F, t119747: F, t125997: F, t126006: F, t126412: F, t1711: F, t1940: F, t2403: F, t27382: F, t27773: F, t27777: F, t27793: F, t27802: F, t27806: F, t27810: F, t27817: F, t31859: F, t31863: F, t31876: F, t33: F, t33727: F, t7207: F, t8490: F, t8494: F) -> (F,) {
    let t127266 = t27799 * t18875;
    let t127284 = t27799 * t126017;
    let t127287 = 3.0 / 2.0 * t2403 * t8490 * t27810 - 3.0 / 2.0 * t2403 * t8494 * t27810 + t1940 * t31876 * t27802 - t1940 * t125997 * t7207 / 2.0 - t126006 + t1940 * t31876 * t27806 + t1940 * t31876 * t27817 + t1940 * t31859 * t1711 / 2.0 - 3.0 / 2.0 * t119747 * t27793 + t1940 * t33727 * t1113 / 2.0 - t1940 * t31863 * t27817 / 2.0 - t1940 * t31863 * t27806 / 2.0 + 3.0 * t119706 * t127266 - t1940 * t31863 * t27802 / 2.0 + t1940 * t126412 * t33 / 2.0 - 3.0 / 2.0 * t2403 * t8494 * t27773 + 3.0 / 2.0 * t2403 * t8490 * t27773 + 3.0 / 2.0 * t2403 * t8490 * t27777 + 2.0 * t27382 * t127284;
    (t127287,)
}
