//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 978/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk978<F: Float>(t13500: F, t633: F, t1324: F, t2: F, t555: F, t2083: F, t4577: F, t7622: F, t100: F, t4661: F, t7629: F, t636: F) -> (F, F, F, F, F, F) {
    let t13501 = t13500 * t633;
    let t13504 = t1324 * t2;
    let t13505 = t13504 * t555;
    let t13510 = t2083 * t4577;
    let t13511 = t13510 * t633;
    let t13515 = -t555 - F::cast_from(3.0_f64) * t7622;
    let t13516 = t100 * t13515;
    let t13525 = t7629 * t4661;
    let t13526 = t13525 * t636;
    (t13501, t13505, t13511, t13515, t13516, t13526)
}
