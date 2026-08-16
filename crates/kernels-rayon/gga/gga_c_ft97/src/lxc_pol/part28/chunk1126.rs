//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1126/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1126(t1882: f64, t35214: f64, t139573: f64, t3483: f64, t35208: f64, t33085: f64, t50260: f64, t12277: f64, t7407: f64, t1017: f64, t32869: f64, t107082: f64, t12968: f64, t1378: f64, t144: f64, t148194: f64, t148196: f64, t148205: f64, t148210: f64, t148219: f64, t148221: f64, t167: f64, t1901: f64, t2185: f64, t23443: f64, t26935: f64, t26981: f64, t26991: f64, t27000: f64, t32992: f64, t33055: f64, t3450: f64, t34853: f64, t3565: f64, t446: f64, t574: f64, t605: f64, t616: f64, t7312: f64, t9016: f64, t95789: f64) -> (f64, f64, f64, f64, f64) {
    let t148223 = t1882 * t35214;
    let t148225 = t139573 * t3483;
    let t148229 = t1882 * t35208;
    let t148234 = t50260 * t33085;
    let t148238 = t12277 * t7407;
    let t148249 = t32869 * t1017;
    let t148254 = -4.0_f64 / 3.0_f64 * t1901 * t107082 * t26981 - 4.0_f64 * t1901 * t9016 * t1378 * t27000 + 4.0_f64 / 3.0_f64 * t1901 * t12968 * t33055 * t3450 - 2.0_f64 / 27.0_f64 * t148194 - 2.0_f64 / 3.0_f64 * t446 * t144 * t148196 + t446 * t574 * t605 * t32992 * t1017 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t144 * t148205 + 2.0_f64 / 3.0_f64 * t446 * t144 * t148210 - 2.0_f64 / 3.0_f64 * t446 * t2185 * t605 * t7312 * t3565 - 2.0_f64 / 9.0_f64 * t148219 + 2.0_f64 / 9.0_f64 * t148221 + 2.0_f64 / 9.0_f64 * t148223 + 2.0_f64 / 3.0_f64 * t446 * t144 * t148225 + 2.0_f64 / 3.0_f64 * t148229 - 2.0_f64 / 9.0_f64 * t1901 * t95789 * t26935 - 2.0_f64 * t446 * t144 * t148234 - t446 * t144 * t148238 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t23443 * t26991 + 2.0_f64 / 3.0_f64 * t446 * t2185 * t616 * t34853 + 2.0_f64 / 3.0_f64 * t446 * t2185 * t167 * t148249;
    (t148225, t148234, t148238, t148249, t148254)
}
