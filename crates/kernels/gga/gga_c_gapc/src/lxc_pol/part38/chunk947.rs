//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 947/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk947<F: Float>(t11831: F, t33338: F, t11764: F, t920: F, t2648: F, t3769: F, t11834: F, t16403: F, t7191: F, t1026: F, t2674: F, t9827: F, t33320: F, t33324: F, t33326: F, t33330: F, t33333: F, t33336: F) -> (F,) {
    let t33339 = t33338 * t11831;
    let t33341 = t11764 * t920;
    let t33343 = t3769 * t2648;
    let t33346 = t11834 * t16403 * t7191;
    let t33349 = t2674 * t1026 * t9827;
    let t33351 = 0.21720231316129303386e-4 * t33320 - 0.34752370105806885418e-3 * t33324 - 0.16217772716043213195e-2 * t33326 + 0.71696352428860134554e-9 * t33330 - 0.11594181388521408695e-4 * t33333 - 0.61454016367594401047e-9 * t33336 + 0.81938688490125868062e-9 * t33339 + 0.16217772716043213195e-2 * t33341 - 0.30660168560756614104e-3 * t33343 + 0.11233430345674682505e-6 * t33346 + 0.57970906942607043474e-5 * t33349;
    (t33351,)
}
