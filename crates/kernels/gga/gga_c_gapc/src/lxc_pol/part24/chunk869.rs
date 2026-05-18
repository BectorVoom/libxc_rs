//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 869/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk869<F: Float>(t8328: F, t8330: F, t8332: F, t8335: F, t8338: F, t8342: F, t8344: F, t8348: F, t8354: F, t8359: F, t8364: F, t8369: F, t8371: F, t8374: F, t8377: F, t8382: F, t8385: F, t8388: F, t8391: F, t8397: F, t8401: F, t8403: F) -> (F, F) {
    let t10445 = F::new(0.20855578275249024918e-2) * t8328 - F::new(0.6951859425083008306e-2) * t8330 - F::new(0.6951859425083008306e-2) * t8332 - F::new(0.43449121406768801913e-4) * t8335 - F::new(0.2534532082061513445e-4) * t8338 - F::new(0.43449121406768801913e-4) * t8342 - F::new(0.86898242813537603826e-4) * t8344 - F::new(0.2534532082061513445e-4) * t8348 - F::new(0.15021326806351236351e-5) * t8354 + F::new(0.1545050757224698596e-4) * t8359 + F::new(0.84484402735383781499e-6) * t8364;
    let t10458 = F::new(0.6951859425083008306e-2) * t8369 + F::new(0.6951859425083008306e-2) * t8371 + F::new(0.41711156550498049836e-2) * t8374 - F::new(0.86898242813537603826e-5) * t8377 + F::new(0.51501691907489953201e-6) * t8382 - F::new(0.1545050757224698596e-4) * t8385 - F::new(0.20855578275249024918e-2) * t8388 - F::new(0.13903718850166016612e-3) * t8391 + F::new(0.14651201313842741887e-4) * t8397 + F::new(0.86898242813537603826e-4) * t8401 - F::new(0.5503555378190714909e-3) * t8403;
    (t10445, t10458)
}
