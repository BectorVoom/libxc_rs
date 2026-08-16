//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 917/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk917(t236: f64, t6776: f64, t2426: f64, t2427: f64, t3758: f64, t209: f64, t8: f64, t1173: f64, t6061: f64, t2567: f64, t6907: f64, t6837: f64, t771: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109200 = t236 * t6776;
    let t109216 = t2426 * t6776;
    let t109230 = t3758 * t2427;
    let t109246 = t8 * t209;
    let t109536 = t6061 * t1173;
    let t109652 = t6907 * t2567;
    let t109713 = t6837 * t771;
    (t109200, t109216, t109230, t109246, t109536, t109652, t109713)
}
