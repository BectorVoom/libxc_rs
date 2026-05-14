//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 685/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk685<F: Float>(t40165: F, t900: F, t9086: F, t20556: F, t587: F, t9438: F, t1645: F, t6949: F, t20700: F, t6710: F, t20551: F, t6914: F, t20696: F, t2476: F, t20561: F, t2487: F) -> (F, F, F, F, F, F, F, F) {
    let t40166 = t900 * t40165;
    let t40186 = t900 * t9086;
    let t40261 = t587 * t9438 * t20556;
    let t40342 = t1645 * t6949;
    let t40372 = t6710 * t9438 * t20700;
    let t40377 = t6914 * t9438 * t20551;
    let t40449 = t2476 * t9438 * t20696;
    let t40452 = t2487 * t9438 * t20561;
    (t40166, t40186, t40261, t40342, t40372, t40377, t40449, t40452)
}
