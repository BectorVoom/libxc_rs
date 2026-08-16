//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1175/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1175<F: Float>(t11522: F, t21778: F, t8677: F, t11523: F, t26226: F, t19670: F, t8681: F, t11526: F, t26778: F, t21655: F, t26369: F, t34419: F, t5541: F) -> (F, F, F, F, F, F) {
    let t34515 = t21778 * t11522 * t8677;
    let t34517 = t11523 * t26226;
    let t34520 = t19670 * t11522 * t8681;
    let t34522 = t11526 * t26778;
    let t34525 = t21655 * t11522 * t26369;
    let t34528 = t5541 * t34419 * t8677;
    (t34515, t34517, t34520, t34522, t34525, t34528)
}
