//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1084/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1084(t34690: f64, t8392: f64, t1851: f64, t7211: f64, t1852: f64, t32457: f64, t979: f64, t102724: f64, t11490: f64, t11810: f64, t137680: f64, t137804: f64, t137872: f64, t137877: f64, t1871: f64, t1901: f64, t23339: f64, t26240: f64, t3205: f64, t3214: f64, t3219: f64, t3238: f64, t32562: f64, t34725: f64, t446: f64, t47120: f64, t5750: f64, t6469: f64, t6547: f64, t83: f64) -> (f64, f64, f64) {
    let t146417 = t8392 * t34690;
    let t146460 = t1851 * t7211;
    let t146473 = t1852 * t32457 * t979;
    let t146483 = -4.0_f64 / 3.0_f64 * t1901 * t11810 * t23339 * t26240 - 4.0_f64 / 3.0_f64 * t1901 * t47120 * t34725 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t102724 * t6547 - 2.0_f64 / 3.0_f64 * t1901 * t11810 * t137804 * t3214 - 2.0_f64 / 3.0_f64 * t1901 * t11490 * t146460 * t3219 + t1901 * t137680 * t3205 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t1871 * t5750 * t6469 + 2.0_f64 / 3.0_f64 * t446 * t83 * t146473 + t137872 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t137877 - 2.0_f64 / 3.0_f64 * t446 * t1871 * t3238 * t32562;
    (t146417, t146473, t146483)
}
