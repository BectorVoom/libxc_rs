//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 848/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk848<F: Float>(t27492: F, t3317: F, t1671: F, t25512: F, t25522: F, t25526: F, t25535: F, t25538: F, t25580: F, t4825: F, t4869: F, t4875: F, t4887: F, t4902: F, t4907: F, t4912: F, t7111: F, t7122: F) -> F {
    let t27498 = t3317 * t27492;
    let t27518 = -F::cast_from(0.42874018118069736972e-3_f64) * t27498 * t4902 - t25535 / F::cast_from(108.0_f64) - t25538 + t7111 * t4887 / F::cast_from(288.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t25580 * t4907 - F::cast_from(0.42874018118069736972e-3_f64) * t25580 * t4912 - F::cast_from(0.28582678745379824648e-3_f64) * t25522 * t4825 + F::cast_from(0.42874018118069736972e-3_f64) * t25512 * t1671 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t4869 - F::cast_from(0.28582678745379824648e-3_f64) * t25522 * t4875 - F::cast_from(0.22866142996303859718e-2_f64) * t25526 * t1671;
    t27518
}
