//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 967/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk967(t8157: f64, t8935: f64, t37931: f64, t8873: f64, t1701: f64, t2059: f64, t7883: f64, t12374: f64, t1683: f64, t1992: f64, t2001: f64, t2030: f64, t2032: f64, t2043: f64, t2057: f64, t2060: f64, t3347: f64, t3392: f64, t39835: f64, t555: f64, t5818: f64, t8825: f64, t8833: f64, t8866: f64, t8998: f64) -> (f64, f64) {
    let t40106 = t8935 * t8157;
    let t40111 = t8873 * t37931;
    let t40123 = t1701 * t7883 * t2059;
    let t40128 = 24.0_f64 * t1992 * t2060 + 24.0_f64 * t2001 * t2057 * t2030 * t2059 - 0.65177969127962413846e0_f64 * t40106 * t555 - 24.0_f64 * t12374 * t8866 + 0.12383814134312858631e2_f64 * t5818 * t40111 - 0.4127938044770952877e1_f64 * t3392 * t40111 + 8.0_f64 * t3347 * t8998 + 0.2416365355361531912e1_f64 * t2043 * t39835 + 0.45910941751869106328e2_f64 * t8825 * t1683 - 0.45910941751869106328e2_f64 * t8833 * t40123 - 0.45910941751869106328e2_f64 * t2032 * t1683;
    (t40123, t40128)
}
