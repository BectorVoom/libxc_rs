//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1409/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1409<F: Float>(t3020: F, t59157: F, t8686: F, t3018: F, t59166: F, t14852: F, t5187: F, t44181: F, t5190: F, t26248: F, t8688: F, t1460: F, t52890: F) -> (F, F, F, F, F, F) {
    let t59176 = F::new(0.57894567559743977359e3) * t8686 * t59157 * t3020;
    let t59179 = F::new(0.48245472966453314466e2) * t3018 * t59166 * t3020;
    let t59181 = F::new(6.0) * t14852 * t5187;
    let t59183 = F::new(0.96490945932906628932e2) * t44181 * t5190;
    let t59186 = F::new(0.620700176468474021e4) * t26248 * t59157 * t8688;
    let t59188 = F::new(4.0) * t52890 * t1460;
    (t59176, t59179, t59181, t59183, t59186, t59188)
}
