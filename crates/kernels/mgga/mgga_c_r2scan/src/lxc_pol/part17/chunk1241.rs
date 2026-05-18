//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1241/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1241<F: Float>(t38646: F, t40137: F, t41709: F, t41721: F, t41722: F, t41725: F, t43586: F, t43588: F, t43592: F, t43594: F, t43597: F, t43599: F) -> F {
    let t44461 = t41709 - F::new(0.16951189180550569635e1) * t40137 - F::new(0.21951497276451705328e-1) * t43586 - F::new(0.86682217400542685632e-1) * t43588 - t41721 + t41722 - t41725 + F::new(0.43663693315433241794e-2) * t43592 - t38646 - F::new(0.17465477326173296718e-1) * t43594 - F::new(0.87327386630866483588e-2) * t43597 - F::new(0.76830240467580968648e0) * t43599;
    t44461
}
