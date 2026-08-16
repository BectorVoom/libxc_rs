//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1218/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1218<F: Float>(t12940: F, t1629: F, t1636: F, t17710: F, t18268: F, t18352: F, t2128: F, t27673: F, t27693: F, t28666: F, t28698: F, t40653: F, t4480: F, t4481: F, t6256: F, t7998: F, t8010: F, t8240: F, t97635: F, t97637: F, t97638: F, t97641: F, t97643: F, t97645: F, t97647: F, t97650: F, t97652: F, t97700: F, t97740: F, t97781: F, t97824: F) -> F {
    let t97845 = -t97635 - F::cast_from(12.0_f64) * t12940 * t28666 * t1636 - t97637 + t97638 - t1629 * (t97700 + t97740 + t97781 + t97824) + F::cast_from(2.0_f64) * t4480 * t27693 * t2128 - t97641 + F::cast_from(4.0_f64) * t4480 * t28698 * t1636 - t7998 * t18352 + F::cast_from(4.0_f64) * t18268 * t27673 - t97643 - t97645 + t97647 + F::cast_from(4.0_f64) * t4480 * t8010 * t6256 + F::cast_from(24.0_f64) * t40653 * t8240 * t4481 + t97650 + t97652 - F::cast_from(2.0_f64) * t17710 * t8010;
    t97845
}
