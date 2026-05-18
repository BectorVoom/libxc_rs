//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1200/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1200<F: Float>(t101: F, t10222: F, t11281: F, t12398: F, t12407: F, t12423: F, t12987: F, t12990: F, t142: F, t159: F, t19107: F, t19121: F, t19157: F, t26131: F, t26145: F, t26153: F, t281: F, t285: F, t33837: F, t33849: F, t3619: F, t3637: F, t3642: F, t3644: F, t3686: F, t42342: F, t42848: F, t43183: F, t48321: F, t48859: F, t524: F, t526: F, t5651: F, t8497: F, t981: F, t988: F) -> F {
    let t48882 = -F::new(0.26861343269868796571e-1) * t26145 + t19107 + F::new(0.23948468020509218188e0) * t26153 - F::new(0.11974234010254609094e-1) * t281 * t48321 * t159 * t285 - F::new(36.0) * t26131 * t5651 * t981 * t3644 - F::new(18.0) * t8497 * t5651 * t981 * t3637 + t101 * t48859 * t526 + t988 * t524 * t12987 * t142 - t19121 + F::new(3.0) * t12990 * t3642 + F::new(0.81358876250083374227e-2) * t33837 - F::new(3.0) * t988 * t42342 * t3619 - F::new(0.71845404061527654564e-1) * t33849 - F::new(3.0) * t988 * t10222 * t12398 + F::new(6.0) * t3686 * t12407 + F::new(0.79828226735030727292e-1) * t42848 + F::new(72.0) * t11281 * t43183 - t19157 + F::new(3.0) * t3686 * t12423;
    t48882
}
