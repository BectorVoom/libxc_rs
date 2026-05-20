//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1608/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1608<F: Float>(t12941: F, t3708: F, t12269: F, t12273: F, t1252: F, t12781: F, t12784: F, t12787: F, t12789: F, t3625: F, t3626: F, t3714: F, t44248: F, t44252: F, t44260: F, t44264: F, t44267: F, t44270: F, t44273: F, t44276: F, t44278: F, t5405: F) -> F {
    let t44280 = t3708 * t12941;
    let t44282 = F::cast_from(0.57165357490759649296e-2_f64) * t3625 * t12787 * t12269 * t5405 + F::cast_from(0.28582678745379824648e-2_f64) * t12784 * t12789 - F::cast_from(0.22866142996303859718e-2_f64) * t44248 + F::cast_from(0.3811023832717309953e-3_f64) * t44252 - F::cast_from(0.34299214494455789577e-2_f64) * t12784 * t12781 - F::cast_from(0.34299214494455789577e-2_f64) * t3625 * t3626 * t12273 * t5405 + F::cast_from(0.17149607247227894789e-2_f64) * t44260 * t3714 + F::cast_from(0.2540682555144873302e-3_f64) * t44264 + F::cast_from(0.85748036236139473944e-3_f64) * t44267 * t1252 - F::cast_from(0.57165357490759649296e-3_f64) * t44270 - F::cast_from(0.28582678745379824648e-3_f64) * t44273 + F::cast_from(0.28582678745379824648e-3_f64) * t44276 + F::cast_from(0.17149607247227894789e-2_f64) * t44278 + F::cast_from(0.17149607247227894789e-2_f64) * t44280;
    t44282
}
