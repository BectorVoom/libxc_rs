//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1248/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1248<F: Float>(t47660: F, t5717: F, t11064: F, t11485: F, t11501: F, t11810: F, t11871: F, t11906: F, t12016: F, t12021: F, t16030: F, t1825: F, t1901: F, t23235: F, t23295: F, t23323: F, t23339: F, t26240: F, t26349: F, t3103: F, t3200: F, t3214: F, t38956: F, t446: F, t452: F, t47659: F, t488: F, t5743: F, t6465: F, t91539: F, t91583: F, t91796: F, t92049: F) -> (F,) {
    let t103252 = t47660 * t5717;
    let t103283 = -2.0 / 3.0 * t1901 * t11810 * t23339 * t11501 + t1901 * t38956 * t6465 / 9.0 + 2.0 / 3.0 * t446 * t452 * t1825 * t26240 + 2.0 / 3.0 * t446 * t452 * t488 * t5743 * t3103 + 4.0 / 9.0 * t47659 * t103252 * t11064 + 4.0 / 9.0 * t47659 * t91539 * t11871 + 2.0 / 9.0 * t1901 * t92049 * t3200 + 2.0 / 9.0 * t1901 * t23323 * t12016 + 4.0 / 9.0 * t1901 * t26349 * t12021 - 2.0 / 27.0 * t91796 + 2.0 / 9.0 * t1901 * t11906 * t23295 + 2.0 / 27.0 * t1901 * t16030 * t23235 - 4.0 / 3.0 * t1901 * t11810 * t91583 * t3214 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t11485;
    (t103283,)
}
