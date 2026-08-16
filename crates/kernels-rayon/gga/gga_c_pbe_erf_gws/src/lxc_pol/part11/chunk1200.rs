//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1200/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1200(t101: f64, t10222: f64, t11281: f64, t12398: f64, t12407: f64, t12423: f64, t12987: f64, t12990: f64, t142: f64, t159: f64, t19107: f64, t19121: f64, t19157: f64, t26131: f64, t26145: f64, t26153: f64, t281: f64, t285: f64, t33837: f64, t33849: f64, t3619: f64, t3637: f64, t3642: f64, t3644: f64, t3686: f64, t42342: f64, t42848: f64, t43183: f64, t48321: f64, t48859: f64, t524: f64, t526: f64, t5651: f64, t8497: f64, t981: f64, t988: f64) -> f64 {
    let t48882 = -0.26861343269868796571e-1_f64 * t26145 + t19107 + 0.23948468020509218188e0_f64 * t26153 - 0.11974234010254609094e-1_f64 * t281 * t48321 * t159 * t285 - 36.0_f64 * t26131 * t5651 * t981 * t3644 - 18.0_f64 * t8497 * t5651 * t981 * t3637 + t101 * t48859 * t526 + t988 * t524 * t12987 * t142 - t19121 + 3.0_f64 * t12990 * t3642 + 0.81358876250083374227e-2_f64 * t33837 - 3.0_f64 * t988 * t42342 * t3619 - 0.71845404061527654564e-1_f64 * t33849 - 3.0_f64 * t988 * t10222 * t12398 + 6.0_f64 * t3686 * t12407 + 0.79828226735030727292e-1_f64 * t42848 + 72.0_f64 * t11281 * t43183 - t19157 + 3.0_f64 * t3686 * t12423;
    t48882
}
