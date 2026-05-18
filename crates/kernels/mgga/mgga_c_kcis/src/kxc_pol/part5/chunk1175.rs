//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1175/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1175<F: Float>(t13410: F, t19727: F, t3200: F, t1010: F, t10466: F, t13382: F, t13391: F, t19674: F, t19682: F, t19686: F, t19689: F, t19692: F, t19696: F, t19700: F, t19704: F, t19708: F, t19713: F, t19717: F, t19720: F, t19725: F, t4978: F, t4981: F, t6302: F, t9563: F) -> (F, F) {
    let t19728 = t13410 * t19727;
    let t19729 = t3200 * t19728;
    let t19732 = -F::new(0.36848765432098765431e-3) * t9563 - F::new(0.13345e0) * t4981 * t4978 - F::new(0.66725e-1) * t19674 * t1010 + F::new(0.890445125e-2) * t10466 * t6302 - F::new(0.55273148148148148147e-2) * t19682 - F::new(0.44218518518518518517e-2) * t19686 + F::new(0.66327777777777777776e-2) * t19689 + F::new(0.33163888888888888888e-2) * t19692 + F::new(0.66327777777777777776e-2) * t19696 - F::new(0.22109259259259259259e-2) * t19700 - F::new(0.22109259259259259259e-2) * t19704 + F::new(0.11054629629629629629e-2) * t19708 + F::new(0.3684876543209876543e-2) * t19713 + F::new(0.66327777777777777776e-2) * t19717 - F::new(0.66327777777777777776e-2) * t19720 - F::new(0.58958024691358024688e-2) * t13382 - F::new(0.7369753086419753086e-3) * t19725 - F::new(0.44218518518518518516e-2) * t19729 + F::new(0.22109259259259259259e-2) * t13391;
    (t19729, t19732)
}
