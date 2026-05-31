//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1126/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1126<F: Float>(t1882: F, t35214: F, t139573: F, t3483: F, t35208: F, t33085: F, t50260: F, t12277: F, t7407: F, t1017: F, t32869: F, t107082: F, t12968: F, t1378: F, t144: F, t148194: F, t148196: F, t148205: F, t148210: F, t148219: F, t148221: F, t167: F, t1901: F, t2185: F, t23443: F, t26935: F, t26981: F, t26991: F, t27000: F, t32992: F, t33055: F, t3450: F, t34853: F, t3565: F, t446: F, t574: F, t605: F, t616: F, t7312: F, t9016: F, t95789: F) -> (F, F, F, F, F) {
    let t148223 = t1882 * t35214;
    let t148225 = t139573 * t3483;
    let t148229 = t1882 * t35208;
    let t148234 = t50260 * t33085;
    let t148238 = t12277 * t7407;
    let t148249 = t32869 * t1017;
    let t148254 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t107082 * t26981 - F::cast_from(4.0_f64) * t1901 * t9016 * t1378 * t27000 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t12968 * t33055 * t3450 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t148194 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148196 + t446 * t574 * t605 * t32992 * t1017 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148205 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148210 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t605 * t7312 * t3565 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148219 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148221 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148223 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148225 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t148229 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t95789 * t26935 - F::cast_from(2.0_f64) * t446 * t144 * t148234 - t446 * t144 * t148238 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t23443 * t26991 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t616 * t34853 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t167 * t148249;
    (t148225, t148234, t148238, t148249, t148254)
}
