//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 998/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk998(t2178: f64, t7407: f64, t1882: f64, t33188: f64, t33057: f64, t8392: f64, t33230: f64, t5: f64, t22914: f64, t34557: f64, t135994: f64, t135996: f64, t137298: f64, t137324: f64, t1564: f64, t22907: f64, t22935: f64, t25558: f64, t25577: f64, t25606: f64, t25612: f64, t25862: f64, t3052: f64, t32011: f64, t32016: f64, t32019: f64, t32425: f64, t5501: f64, t6418: f64, t7824: f64, t925: f64) -> (f64, f64, f64, f64, f64) {
    let t140419 = t2178 * t7407;
    let t140424 = t1882 * t33188;
    let t140426 = t8392 * t33057;
    let t140469 = t5 * t33230;
    let t144308 = t22914 * t34557;
    let t144337 = -t144308 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t5501 * t22907 * t25862 - t137324 * t6418 / 18.0_f64 + t5501 * t7824 * t137298 * t925 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t25577 * t7824 * t32019 * t3052 + t135994 / 54.0_f64 + t32016 * t25606 / 9.0_f64 + t32016 * t25612 / 9.0_f64 - t25577 * t1564 * t32011 * t3052 / 9.0_f64 + t22935 * t34557 / 9.0_f64 - t25558 * t32425 / 9.0_f64 + t135996 / 27.0_f64;
    (t140419, t140424, t140426, t140469, t144337)
}
