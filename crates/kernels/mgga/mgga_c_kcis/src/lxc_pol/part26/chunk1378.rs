//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1378/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1378<F: Float>(t103626: F, t7898: F, t18210: F, t2237: F, t29343: F, t102640: F, t102642: F, t27410: F, t28480: F, t29300: F, t29393: F, t7916: F, t8148: F, t98777: F, t98795: F, t98804: F, t98806: F, t98813: F) -> F {
    let t103670 = t7898 * t103626;
    let t103674 = t2237 * t18210 * t29343;
    let t103686 = F::new(0.30918233506944444444e-4) * t103670 - F::new(0.49745833333333333332e-2) * t102640 + F::new(0.23168402777777777778e-3) * t103674 + F::new(0.69505208333333333333e-3) * t29393 * t7916 - F::new(0.30891203703703703704e-3) * t98777 - F::new(0.58958024691358024689e-2) * t102642 + F::new(0.92754700520833333333e-4) * t27410 * t29300 + t98795 + F::new(0.11054629629629629629e-2) * t98804 - F::new(0.7369753086419753086e-3) * t98806 - F::new(0.37069444444444444444e-2) * t28480 * t8148 + t98813;
    t103686
}
