//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1220/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1220<F: Float>(t17323: F, t27494: F, t27503: F, t48058: F, t12940: F, t18268: F, t2268: F, t27697: F, t27705: F, t28655: F, t28698: F, t40556: F, t40662: F, t4475: F, t4481: F, t51097: F, t52930: F, t52955: F, t6225: F, t8001: F, t8240: F, t8251: F, t94824: F, t97654: F, t97657: F, t97852: F, t97854: F, t97856: F, t97862: F, t97870: F) -> (F, F, F) {
    let t97875 = F::cast_from(4.0_f64) * t27494 * t17323;
    let t97877 = F::cast_from(6.0_f64) * t48058 * t27503;
    let t97880 = -F::cast_from(6.0_f64) * t12940 * t4481 * t8251 + F::cast_from(2.0_f64) * t18268 * t27697 - t2268 * t52955 - F::cast_from(6.0_f64) * t27705 * t51097 - F::cast_from(12.0_f64) * t28655 * t40662 - F::cast_from(2.0_f64) * t28698 * t4475 + F::cast_from(2.0_f64) * t40556 * t8240 + F::cast_from(4.0_f64) * t52930 * t8001 + F::cast_from(4.0_f64) * t6225 * t94824 - t97654 - t97657 + t97852 + t97854 - t97856 - t97862 - t97870 - t97875 + t97877;
    (t97875, t97877, t97880)
}
