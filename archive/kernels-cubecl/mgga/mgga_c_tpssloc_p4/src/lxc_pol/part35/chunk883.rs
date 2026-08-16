//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 883/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk883<F: Float>(t2371: F, t4199: F, t1409: F, t2517: F, t707: F, t1484: F, t212: F, t9523: F, t2586: F, t1489: F, t9541: F, t4134: F, t9546: F) -> (F, F, F, F, F) {
    let t12943 = t4199 * t2371;
    let t12945 = t2517 * t1409;
    let t12946 = t707 * t12945;
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    let t12986 = t2586 * t12985;
    let t13010 = t9541 * t1489;
    let t13022 = t9546 * t4134;
    (t12943, t12946, t12986, t13010, t13022)
}
