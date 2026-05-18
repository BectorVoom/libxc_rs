//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1078/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1078<F: Float>(t33320: F, t33324: F, t33326: F, t33330: F, t33333: F, t33336: F, t33339: F, t33341: F, t33343: F, t33346: F, t33349: F, t11902: F, t19161: F) -> (F, F) {
    let t33351 = F::new(0.21720231316129303386e-4) * t33320 - F::new(0.34752370105806885418e-3) * t33324 - F::new(0.16217772716043213195e-2) * t33326 + F::new(0.71696352428860134554e-9) * t33330 - F::new(0.11594181388521408695e-4) * t33333 - F::new(0.61454016367594401047e-9) * t33336 + F::new(0.81938688490125868062e-9) * t33339 + F::new(0.16217772716043213195e-2) * t33341 - F::new(0.30660168560756614104e-3) * t33343 + F::new(0.11233430345674682505e-6) * t33346 + F::new(0.57970906942607043474e-5) * t33349;
    let t33353 = t11902 * t19161;
    (t33351, t33353)
}
