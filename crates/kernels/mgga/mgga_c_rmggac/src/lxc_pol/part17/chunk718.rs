//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 718/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk718<F: Float>(t10205: F, t8500: F, t8692: F, t8698: F, t9037: F, t9040: F, t9060: F, t9062: F, t9075: F, t9079: F, t9091: F, t117: F, t5011: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10206 = F::new(0.11974241701863808564e0) * t10205;
    let t10280 = F::new(0.39726959900411316772e-4) * t8500;
    let t10357 = F::new(0.39726959900411316772e-4) * t8692;
    let t10360 = F::new(0.39726959900411316772e-4) * t8698;
    let t10383 = F::new(0.49658699875514145965e-4) * t9037;
    let t10384 = F::new(0.39726959900411316772e-4) * t9040;
    let t10385 = F::new(0.47896966807455234256e0) * t9060;
    let t10386 = F::new(0.3193131120497015617e0) * t9062;
    let t10487 = F::new(0.15965655602485078085e0) * t9075;
    let t10496 = F::new(0.15965655602485078085e0) * t9079;
    let t10504 = F::new(0.39726959900411316772e-4) * t9091;
    let t11905 = t5011 * t117;
    (t10206, t10280, t10357, t10360, t10383, t10384, t10385, t10386, t10487, t10496, t10504, t11905)
}
