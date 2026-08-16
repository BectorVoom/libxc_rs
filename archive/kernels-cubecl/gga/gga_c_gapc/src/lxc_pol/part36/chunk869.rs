//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 869/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk869<F: Float>(t8328: F, t8330: F, t8332: F, t8335: F, t8338: F, t8342: F, t8344: F, t8348: F, t8354: F, t8359: F, t8364: F, t8369: F, t8371: F, t8374: F, t8377: F, t8382: F, t8385: F, t8388: F, t8391: F, t8397: F, t8401: F, t8403: F) -> (F, F) {
    let t10445 = F::cast_from(0.20855578275249024918e-2_f64) * t8328 - F::cast_from(0.6951859425083008306e-2_f64) * t8330 - F::cast_from(0.6951859425083008306e-2_f64) * t8332 - F::cast_from(0.43449121406768801913e-4_f64) * t8335 - F::cast_from(0.2534532082061513445e-4_f64) * t8338 - F::cast_from(0.43449121406768801913e-4_f64) * t8342 - F::cast_from(0.86898242813537603826e-4_f64) * t8344 - F::cast_from(0.2534532082061513445e-4_f64) * t8348 - F::cast_from(0.15021326806351236351e-5_f64) * t8354 + F::cast_from(0.1545050757224698596e-4_f64) * t8359 + F::cast_from(0.84484402735383781499e-6_f64) * t8364;
    let t10458 = F::cast_from(0.6951859425083008306e-2_f64) * t8369 + F::cast_from(0.6951859425083008306e-2_f64) * t8371 + F::cast_from(0.41711156550498049836e-2_f64) * t8374 - F::cast_from(0.86898242813537603826e-5_f64) * t8377 + F::cast_from(0.51501691907489953201e-6_f64) * t8382 - F::cast_from(0.1545050757224698596e-4_f64) * t8385 - F::cast_from(0.20855578275249024918e-2_f64) * t8388 - F::cast_from(0.13903718850166016612e-3_f64) * t8391 + F::cast_from(0.14651201313842741887e-4_f64) * t8397 + F::cast_from(0.86898242813537603826e-4_f64) * t8401 - F::cast_from(0.5503555378190714909e-3_f64) * t8403;
    (t10445, t10458)
}
