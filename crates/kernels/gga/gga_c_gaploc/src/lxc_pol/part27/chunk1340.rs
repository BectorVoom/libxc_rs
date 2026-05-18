//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1340/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1340<F: Float>(t34862: F, t2478: F, t6583: F, t8272: F, t2890: F, t7047: F, t20670: F, t20671: F, t26922: F, t21283: F, t7035: F, t10402: F, t4811: F) -> (F, F, F, F, F, F) {
    let t34863 = F::new(0.59584149919750711116e-1) * t34862;
    let t34865 = t6583 * t8272 * t2478;
    let t34866 = F::new(0.38342925953920749676e0) * t34865;
    let t34868 = t6583 * t2890 * t7047;
    let t34869 = F::new(0.19171462976960374838e0) * t34868;
    let t34873 = t20670 * t20671 * t26922;
    let t34874 = F::new(0.85206502119823888168e-1) * t34873;
    let t34876 = t21283 * t2890 * t7035;
    let t34877 = F::new(0.38342925953920749676e0) * t34876;
    let t34878 = t4811 * t10402;
    (t34863, t34866, t34869, t34874, t34877, t34878)
}
