//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 900/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk900<F: Float>(t1550: F, t2060: F, t30400: F, t194: F, t1979: F, t1982: F, t201: F, t6070: F, t2320: F, t40359: F, t6355: F, t8404: F) -> (F, F, F, F) {
    let t44941 = t1550 * t2060 * t30400;
    let t44949 = t194 * t6070 * t201 * t1979 * t1982;
    let t44951 = t40359 * t2320;
    let t44954 = t6355 * t8404;
    (t44941, t44949, t44951, t44954)
}
