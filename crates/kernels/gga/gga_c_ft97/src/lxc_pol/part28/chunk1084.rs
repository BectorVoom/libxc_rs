//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1084/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1084<F: Float>(t34690: F, t8392: F, t1851: F, t7211: F, t1852: F, t32457: F, t979: F, t102724: F, t11490: F, t11810: F, t137680: F, t137804: F, t137872: F, t137877: F, t1871: F, t1901: F, t23339: F, t26240: F, t3205: F, t3214: F, t3219: F, t3238: F, t32562: F, t34725: F, t446: F, t47120: F, t5750: F, t6469: F, t6547: F, t83: F) -> (F, F, F) {
    let t146417 = t8392 * t34690;
    let t146460 = t1851 * t7211;
    let t146473 = t1852 * t32457 * t979;
    let t146483 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11810 * t23339 * t26240 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t47120 * t34725 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11490 * t102724 * t6547 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t11810 * t137804 * t3214 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t11490 * t146460 * t3219 + t1901 * t137680 * t3205 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t5750 * t6469 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t146473 + t137872 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t137877 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t3238 * t32562;
    (t146417, t146473, t146483)
}
