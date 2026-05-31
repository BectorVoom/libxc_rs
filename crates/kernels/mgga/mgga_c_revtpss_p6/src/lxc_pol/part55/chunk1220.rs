//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1220/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1220<F: Float>(t34097: F, t890: F, t1940: F, t2255: F, t8657: F, t102851: F, t110165: F, t121751: F, t125962: F, t126031: F, t1468: F, t26425: F, t27376: F, t27391: F, t28460: F, t28472: F, t31873: F, t32487: F, t32491: F, t32498: F, t32506: F, t34098: F, t98658: F, t98785: F) -> (F, F, F) {
    let t127914 = t34097 * t890;
    let t127929 = t1940 * t8657 * t2255;
    let t127939 = -F::cast_from(3.0_f64) * t28472 * t98785 * t127914 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t126031 + t110165 * t32506 + t1940 * t32487 * t1468 / F::cast_from(2.0_f64) + t102851 * t34098 - t1940 * t32491 * t27391 / F::cast_from(2.0_f64) + t127929 + t28472 * t125962 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t98658 * t32498 - t1940 * t28460 * t31873 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t121751 * t27376;
    (t127914, t127929, t127939)
}
